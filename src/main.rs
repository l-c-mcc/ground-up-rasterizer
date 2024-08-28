mod color;
mod geometry;
mod rasterizer;
mod timer;

use color::{Color, Rgba};
use geometry::triangle;
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
    let triangle = triangle();
    let mut buffer = vec![u32::from(&Rgba::from(&Color::Black)); width * height];
    let draw_buffer = rasterize_geometry(&vec![triangle]).unwrap();
    for obj in draw_buffer {
        buffer[xy_to_1d(obj.x, obj.y, width as i32)] = u32::from(&obj.color);
    }

    let mut window = Window::new("Rasterizer", width, height, WindowOptions::default()).unwrap();
    while window.is_open() {
        window.update_with_buffer(&buffer, width, height).unwrap();
    }
}

fn xy_to_1d(x: i32, y: i32, width: i32) -> usize {
    (y * width + x) as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xy_to_1d() {
        assert_eq!(xy_to_1d(0, 0, 500), 0);
        assert_eq!(xy_to_1d(0, 1, 500), 500);
        assert_eq!(xy_to_1d(499, 499, 500), 249999);
    }
}
