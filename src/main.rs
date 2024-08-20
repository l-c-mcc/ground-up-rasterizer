mod color;
mod geometry;
mod rasterizer;

use color::RGBA;
use minifb::{Window, WindowOptions};

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
    let height = 720;
    let buffer = vec![u32::from(&RGBA::color(0.5, 0.5, 0.5)); width * height];

    let mut window = Window::new("Rasterizer", width, height, WindowOptions::default()).unwrap();
    while window.is_open() {
        window.update_with_buffer(&buffer, width, height).unwrap();
    }
}
