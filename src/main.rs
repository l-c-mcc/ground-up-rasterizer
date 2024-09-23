// Remove once in 3D?
#![allow(dead_code)]

mod color;
mod geometry;
mod math;
mod rasterizer;
mod timer;

use std::f32::consts::PI;

use nalgebra as na;
use color::{Color, Rgba};
use geometry::{triangle, GeoError};
use minifb::{Window, WindowOptions};
use rasterizer::rasterize_geometry;
use timer::Timer;

/*
rough rendering pipeline:
- Input assembler (handle geometry)
- vertex shader
- rasterize
- pixel shading

Implementation order?
1. handling the idea of vertices with associated info (color?) in
   3d space within the code
        a. Vertices can be used to form lines and triangles.
2. local space -> world space -> view space transformations -> canonical view volume -> window
    local space -> world space: transform/scale/rotate each object as neeeded
    world space -> view space: matrix transform all points
    view space -> canonical view volume: more matrix transformations?
    canonical view volume -> window: scale view volume by resolution
3. rasterization

Rasterization to-do
1. Geometry object needs to more gracefully handle vertices
2. Test line objects with >2 points
3. Triangle rasterization
4. Line clipping
 */

fn main() {
    let width = 1000;
    let height = 1000;
    let mut _timer = Timer::default();
    let mut shape = triangle();
    let translation = math::translation_matrix(shape.vec_from_origin().unwrap());
    let translation2 = math::translation_matrix(shape.vec_from_origin().unwrap() * -1.0);
    let rotation = math::y_rotation_matrix(PI / 2.5);
    let scalar = math::scale_matrix(na::Vector3::new(10.0, -10.5, 0.5));
    shape.transform(translation2);
    shape.transform(rotation);
    shape.transform(scalar);
    shape.transform(translation);
    let mut buffer = vec![u32::from(&Rgba::from(&Color::Black)); width * height];
    let mut draw_buffer = vec![];
    draw_buffer.append(
        &mut rasterize_geometry(&vec![shape]).unwrap_or_else(|error| {
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

    let mut window = Window::new("Rasterizer", width, height, WindowOptions::default()).unwrap();
    while window.is_open() {
        window.update_with_buffer(&buffer, width, height).unwrap();
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
