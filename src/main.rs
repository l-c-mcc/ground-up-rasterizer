// Remove once in 3D?
#![allow(dead_code, unused_imports)]

mod color;
mod geometry;
mod math;
mod rasterizer;
mod timer;
mod world;

use std::cell::RefCell;
use std::f32::{consts::PI, INFINITY};

use color::{Color, Rgba};
use geometry::{cube, direction, line, point, right_triangle, square, triangle, GeoError, Geometry};
use math::{f32_equals, translation_matrix, OrdFloat};
use minifb::{Key, Window, WindowOptions};
use nalgebra as na;
use rasterizer::rasterize_geometry;
use timer::Timer;
use world::{Camera, World};

fn main() {
    let mut timer = Timer::default();

    let width = 1500;
    let height = 1000;
    let mut world = World::default();
    let mut camera = Camera::new(-200., -200., width as f32, height as f32, 0.0);
    let mut window = Window::new("Rasterizer", width, height, WindowOptions::default()).unwrap();
    // Triangle depth and alpha testing
    let mut t1 = triangle();
    t1.set_name(Some("Solid triangle for transparency testing".to_string()));
    t1.scale(na::matrix![200.0;200.0;0.0]);
    let mut t2 = t1.clone();
    t2.set_name(Some("Lower depth transparency triangle".to_string()));
    let mut t3 = t2.clone();
    t3.set_name(Some("Higher depth transparency triangle".to_string()));
    t1.translate(direction(0.0, 0.0, 0.0));
    t2.translate(direction(50.0, 50.0, 1.0));
    t3.translate(direction(100.0, 100.0, 2.0));
    t1.set_color(Color::Custom(0.0,1.0,1.0,1.0));
    t2.set_color(Color::Custom(0.0, 0.0, 1.0, 0.5));
    t3.set_color(Color::Custom(0.0, 1.0, 0.0, 0.45));
    world.insert(t1);
    world.insert(t2);
    world.insert(t3);
    // Line depth testing
    let mut l1 = line();
    l1.scale(na::matrix![200.0;200.0;1.0]);
    l1.translate(point(-200.0, 600.0, 0.0));
    l1.set_name(Some("Horizontal line".to_string()));
    let mut l2 = l1.clone();
    l2.rotation(0.0, 0.0, (2.0 * PI) / 3.0);
    l2.translate(direction(120.0, -170.0, 0.0));
    l2.set_name(Some("Rotated line 2pi/3".to_string()));
    let mut l3 = l1.clone();
    l3.set_name(Some("Rotated line 4pi/3".to_string()));
    l3.rotation(0.0, 0.0, (4.0 * PI) / 3.0);
    l3.translate(direction(180.0, 0.0, 0.0));
    l1.translate(direction(0.0, -20.0, 0.0));
    world.insert(l1);
    world.insert(l2);
    world.insert(l3);
    let mut s = square();
    s.scale(na::matrix![200.0;200.0;200.0]);
    s.translate(direction(500.0, 500.0, 0.0));
    s.set_name(Some("Pointing square".to_string()));
    let mut t = triangle();
    t.scale(na::matrix![200.0; -200.0; 0.0]);
    t.rotation(0.0, 0.0, PI);
    t.translate(direction(500.0, -200.0, 0.0));
    s.set_name(Some("Triangle above pointing square".to_string()));
    // t.set_animation(|geo: &mut Geometry, time: f32| {
    //     geo.rotation(0.0, 0.0, time * 2.0);
    //     let scale = 100.0 * time.sin();
    //     geo.scale(na::matrix![scale; scale; 0.0]);
    //     let pos_x = 300.0 * time.cos();
    //     let pos_y = 300.0 * time.sin();
    //     geo.set_position(point(pos_x, pos_y, 0.0));
    // });
    world.insert(t);
    world.insert(s);
    // 3d testing
    let mut cube = cube();
    cube.translate(direction(1000.0, 1000.0, 0.0));
    cube.scale(na::matrix![250.0;250.0;250.0]);
    cube.rotation(0.0, (90.0 as f32).to_radians(), 0.0);
    cube.set_animation(|geo, time| {
         geo.rotation(time / 2.0, time, 0.0);
    });
    world.insert(cube);
    let push_back = direction(0.0,0.0, -400.0);
    for obj in &mut world.objects {
        obj.translate(push_back);
    }
    let mut cur = 0;
    let mut rgba_buffer = vec![Rgba::from(&Color::Black); width * height];
    let mut depth_buffer = vec![OrdFloat(-f32::INFINITY); width * height];
    let mut draw_buffer = vec![];
    let mut u32_buffer: Vec<u32> = vec![0; width * height];
    let mut opaque = vec![];
    let mut transparent = vec![];
    let mut fps_sum = 0.;
    let mut fps_count = 0.;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // update timer
        timer.tick();
        let delta_time = timer.delta_time_secs();
        let current_time = timer.time_elapsed_secs();
        // fps count
        cur += 1;
        if cur % 60 == 0 {
            let cur_fps = 1.0 / delta_time;
            fps_sum += cur_fps;
            fps_count += 1.;
            println!(
                "{} fps; {} delta time; {} average fps",
                cur_fps,
                delta_time,
                fps_sum / fps_count
            );
        }
        // camera movement
        let angle = rotate_camera(&window);
        camera.add_rotation(angle * delta_time);
        let (x, y) = move_camera(&window);
        let x = x * delta_time;
        let y = y * delta_time;
        camera.translate(x, y);
        // render
        let to_render = camera.world_view(&world, width as f32, height as f32, current_time);
        for obj in &to_render {
            rasterize_geometry(obj, &mut draw_buffer).unwrap_or_else(|error| {
                match error {
                    GeoError::NotDiv3(_) => {
                        eprintln!("The number of vertices of a triangle is not divisible by 3");
                    }
                };
            });
        }
        let db_len = draw_buffer.len();
        for i in 0..db_len {
            if draw_buffer[i].color.a == OrdFloat(1.0) {
                opaque.push(i);
            } else {
                transparent.push(i);
            }
        }
        for i in opaque.iter() {
            let obj = &draw_buffer[*i];
            if let Some(index) = xy_to_1d(obj.x, obj.y, width as i32, height as i32) {
                if obj.depth > depth_buffer[index] {
                    rgba_buffer[index] = obj.color.clone();
                    depth_buffer[index] = obj.depth;
                }
            }
        }
        // layer transparent on top of opaque
        transparent.sort_unstable_by_key(|cur| draw_buffer[*cur].depth);
        for i in transparent.iter() {
            let obj = &draw_buffer[*i];
            if let Some(index) = xy_to_1d(obj.x, obj.y, width as i32, height as i32) {
                if obj.depth > depth_buffer[index] {
                    rgba_buffer[index].over_blend(obj.color.clone());
                    // does not need to be updated because sorted; maybe remove?
                    depth_buffer[index] = obj.depth;
                }
            }
        }
        for i in 0..(width * height) {
            u32_buffer[i] = u32::from(&rgba_buffer[i]);
        }
        window
            .update_with_buffer(&u32_buffer, width, height)
            .unwrap();
        // reset buffers
        for item in &mut rgba_buffer {
            *item = Rgba::color(0.0, 0.0, 0.0);
        }
        for item in &mut depth_buffer {
            *item = OrdFloat(-f32::INFINITY);
        }
        for item in &mut u32_buffer {
            *item = 0;
        }
        transparent.clear();
        opaque.clear();
        draw_buffer.clear();
    }
}

fn move_camera(window: &Window) -> (f32, f32) {
    use Key::{A, D, S, W};
    let speed = 500.0;
    let mut x_vel = 0.0;
    let mut y_vel = 0.0;
    let move_options = vec![
        (W, 0.0, -speed),
        (A, -speed, 0.0),
        (S, 0.0, speed),
        (D, speed, 0.0),
    ];
    for (opt, x, y) in move_options {
        if window.is_key_down(opt) {
            x_vel += x;
            y_vel += y;
        }
    }
    let sqrt2 = 2.0_f32.sqrt();
    if !f32_equals(x_vel, 0.0) && !f32_equals(y_vel, 0.0) {
        x_vel /= sqrt2;
        y_vel /= sqrt2;
    }
    (x_vel, y_vel)
}

fn rotate_camera(window: &Window) -> f32 {
    use Key::{E, Q};
    let speed = PI / 15.0;
    match (window.is_key_down(Q), window.is_key_down(E)) {
        (true, false) => -speed,
        (false, true) => speed,
        _ => 0.0,
    }
}

fn xy_to_1d(x: i32, y: i32, width: i32, height: i32) -> Option<usize> {
    if x >= width || x < 0 || y < 0 || y >= height {
        None
    } else {
        Some((y * width + x) as usize)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xy_to_1d() {
        let height = 500;
        let width = 500;
        assert_eq!(xy_to_1d(0, 0, width, height), Some(0));
        assert_eq!(xy_to_1d(0, 1, width, height), Some(500));
        assert_eq!(xy_to_1d(499, 499, width, height), Some(249999));
        assert_eq!(xy_to_1d(500, 499, width, height), None);
    }
}
