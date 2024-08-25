mod color;
mod geometry;
mod rasterizer;

use color::{Color, Rgba};
use geometry::line;
use minifb::{Window, WindowOptions};
use rasterizer::rasterize_geometry;

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
 */

fn main() {
    let width = 1280;
    let height = 1000;
    let mut buffer = vec![u32::from(&Rgba::from(&Color::Black)); width * height];

    let mut window = Window::new("Rasterizer", width, height, WindowOptions::default()).unwrap();
    let line1 = line();
    let draw_buffer = rasterize_geometry(vec![line1]);
    for obj in draw_buffer {
        buffer[xy_to_1d(obj.x, obj.y, width as i32)] = u32::from(&Rgba::from(&obj.color));
    }
    while window.is_open() {
        window.update_with_buffer(&buffer, width, height).unwrap();
    }
}

fn xy_to_1d(x: i32, y: i32, width: i32) -> usize {
    (y * width + x) as usize
}
