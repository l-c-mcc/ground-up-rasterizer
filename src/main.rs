use minifb::{Window, WindowOptions};

fn main() {
    let width = 500;
    let height = 500;
    let buffer: Vec<u32> = vec![255 << 16 | 255 << 8 | 255; width * height];

    let mut window = Window::new("Rasterizer", width, height, WindowOptions::default()).unwrap();
    while window.is_open() {
        window.update_with_buffer(&buffer, width, height).unwrap();
    }
}
