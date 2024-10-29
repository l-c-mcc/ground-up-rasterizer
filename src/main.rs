// Remove once in 3D?
#![allow(dead_code, unused_imports)]

mod color;
mod geometry;
mod math;
mod rasterizer;
mod timer;
mod world;

use std::f32::consts::PI;

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
    let mut camera = Camera::new(width as f32, height as f32, 0.0);
    let mut window = Window::new("Rasterizer", width, height, WindowOptions::default()).unwrap();
    // Triangle depth and alpha testing
    let mut t1 = triangle();
    t1.scale(na::matrix![200.0;200.0;0.0]);
    let mut t2 = t1.clone();
    let mut t3 = t2.clone();
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
    let mut l2 = l1.clone();
    l2.rotation(0.0, 0.0, (2.0 * PI) / 3.0);
    l2.translate(direction(120.0, -170.0, 0.0));
    let mut l3 = l1.clone();
    l3.rotation(0.0, 0.0, (4.0 * PI) / 3.0);
    l3.translate(direction(180.0, 0.0, 0.0));
    l1.translate(direction(0.0, -20.0, 0.0));
    world.insert(l1);
    world.insert(l2);
    world.insert(l3);
    let mut s = square();
    s.scale(na::matrix![200.0;200.0;200.0]);
    s.translate(direction(500.0, 500.0, 0.0));
    let mut t = triangle();
    t.scale(na::matrix![200.0; -200.0; 0.0]);
    t.rotation(0.0, 0.0, PI);
    t.translate(direction(500.0, -200.0, 0.0));
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
    let transform_back = translation_matrix(direction(0.0,0.0, -400.0));
    for obj in &mut world.objects {
        obj.transform(transform_back);
    }
    while window.is_open() {
        timer.tick();
        let delta_time = timer.delta_time_secs();
        let current_time = timer.time_elapsed_secs();
        let angle = rotate_camera(&window);
        camera.add_rotation(angle * delta_time);
        let (x, y) = move_camera(&window);
        let x = x * delta_time;
        let y = y * delta_time;
        camera.translate(x, y);
        let to_render = camera.world_view(&world, width as f32, height as f32, current_time);
        let mut buffer = vec![Rgba::from(&Color::Black); width * height];
        let mut depth = vec![OrdFloat(-f32::INFINITY); width * height];
        let mut draw_buffer = vec![];
        for obj in &to_render {
            draw_buffer.append(&mut rasterize_geometry(obj).unwrap_or_else(|error| {
                match error {
                    GeoError::NotDiv3(_) => {
                        eprintln!("The number of vertices of a triangle is not divisible by 3",)
                    }
                };
                vec![]
            }));
        }
        let (opaque, mut transparent) =
            draw_buffer
                .into_iter()
                .fold((vec![], vec![]), |mut acc, cur| {
                    if cur.color.a == OrdFloat(1.0) {
                        acc.0.push(cur);
                    } else {
                        acc.1.push(cur);
                    }
                    acc
                });
        // fill in greatest depth opaque values
        for obj in opaque {
            if let Some(index) = xy_to_1d(obj.x, obj.y, width as i32, height as i32) {
                if obj.depth > depth[index] {
                    buffer[index] = obj.color;
                    depth[index] = obj.depth;
                }
            }
        }
        // layer transparent on top of opaque
        transparent.sort_unstable_by_key(|cur| cur.depth);
        for obj in transparent {
            if let Some(index) = xy_to_1d(obj.x, obj.y, width as i32, height as i32) {
                if obj.depth > depth[index] {
                    buffer[index].over_blend(obj.color);
                    // does not need to be updated because sorted; maybe remove?
                    depth[index] = obj.depth;
                }
            }
        }
        let buffer: Vec<u32> = buffer.into_iter().map(|cur| u32::from(&cur)).collect();
        window.update_with_buffer(&buffer, width, height).unwrap();
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
