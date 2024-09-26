// Remove once in 3D?
#![allow(dead_code)]

mod color;
mod geometry;
mod math;
mod rasterizer;
mod timer;
mod world;

use color::{Color, Rgba};
use geometry::{triangle, GeoError};
use math::f32_equals;
use minifb::{Key, Window, WindowOptions};
use nalgebra as na;
use rasterizer::rasterize_geometry;
use timer::Timer;
use world::{Camera, World};

fn main() {
    let mut timer = Timer::default();

    let width = 1000;
    let height = 1000;
    let mut world = World::default();
    let mut camera = Camera::new(width as f32, height as f32);
    let t1 = triangle();
    let mut t2 = triangle();
    let mut t3 = triangle();
    t2.transform(math::translation_matrix(na::Vector3::new(
        (width + 10) as f32,
        0.0,
        0.0,
    )));
    t3.transform(math::translation_matrix(na::Vector3::new(
        (width / 2) as f32,
        0.0,
        0.0,
    )));
    world.insert(t1);
    world.insert(t2);
    world.insert(t3);

    let mut window = Window::new("Rasterizer", width, height, WindowOptions::default()).unwrap();
    while window.is_open() {
        timer.tick();
        let delta_time = timer.delta_time_secs();
        let (x, y) = move_camera(&window);
        let x = x * delta_time;
        let y = y * delta_time;
        camera.translate(x, y);
        let to_render = camera.world_view(&world, width as f32, height as f32);
        let mut buffer = vec![u32::from(&Rgba::from(&Color::Black)); width * height];
        let mut draw_buffer = vec![];
        draw_buffer.append(
            &mut rasterize_geometry(&to_render, camera.position()).unwrap_or_else(|error| {
                match error {
                    GeoError::NotDiv3(geo) => eprintln!(
                    "The number of vertices of the following triangle is not divisible by 3: {:?}",
                    geo
                ),
                    e => panic!("{:?}", e),
                };
                vec![]
            }),
        );
        for obj in draw_buffer {
            if let Some(index) = xy_to_1d(obj.x, obj.y, width as i32, height as i32) {
                buffer[index] = u32::from(&obj.color);
            }
        }
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
