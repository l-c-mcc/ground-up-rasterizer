use crate::color::Color;
use crate::geometry::{Geometry, GeometryType};
use std::mem::swap;

pub struct ToDraw {
    pub x: i32,
    pub y: i32,
    pub color: Color,
    _depth: f32,
}

impl ToDraw {
    fn new(x: i32, y: i32, color: Color) -> Self {
        Self {
            x,
            y,
            color,
            _depth: 0.0,
        }
    }
}

// to-do: handle depth
pub fn rasterize_geometry(geometry: Vec<Geometry>) -> Vec<ToDraw> {
    let mut draw_buffer = vec![];
    for obj in geometry {
        match obj.geo_type {
            GeometryType::Line => draw_buffer.extend(draw_line(obj)),
            GeometryType::Triangle => todo!(),
        }
    }
    draw_buffer
}


/// Uses Bresenham's; look into Wu's for anti-aliasing
/// To-do: handle vert/horiz lines
/// To-do: Turn into Result?
/// To-do: swap is x0 > x1
/// To-do: color interp
fn draw_line(line: Geometry) -> Vec<ToDraw> {
    let mut vertex1 = &line.vertex_locations[line.vertices[0].index];
    let mut vertex1_color = &line.vertices[0].color;
    let mut vertex2 = &line.vertex_locations[line.vertices[1].index];
    let mut vertex2_color = &line.vertices[1].color;
    if vertex1[0] > vertex2[0] {
        swap(&mut vertex1, &mut vertex2);
        swap(&mut vertex1_color, &mut vertex2_color);
    }
    let y_diff = vertex2[1] - vertex1[1];
    let x_diff = vertex2[0] - vertex1[0];
    let slope = y_diff / x_diff;
    let y_intercept = (vertex1[1] - slope * vertex1[0]).round() as i32;
    let imp_line = |x: i32, y: i32| {
        (y_diff as i32 * x) - (x_diff as i32 * y) + (x_diff as i32 * y_intercept)
    };
    let mut y = vertex1[1] as i32;
    let mut draw_buffer = vec![];
    for x in (vertex1[0].round() as i32)..=(vertex2[0].round() as i32) {
        let d = imp_line(2*x, y);
        if d > 0 {
            y += 1;
        }
        draw_buffer.push(ToDraw::new(x, y, *vertex1_color));
    }
    draw_buffer
}
