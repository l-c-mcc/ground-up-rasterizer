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
/// To-do: color interp
fn draw_line(line: Geometry) -> Vec<ToDraw> {
    let vertex1 = &line.vertex_locations[line.vertices[0].index];
    let vertex2 = &line.vertex_locations[line.vertices[1].index];
    let vertex1_color = &line.vertices[0].color;
    let vertex2_color = &line.vertices[1].color;
    let mut x0 = vertex1[0];
    let mut y0 = vertex1[1];
    let mut x1 = vertex2[0];
    let mut y1 = vertex2[1];
    let mut y_diff = y1 - y0;
    let mut x_diff = x1 - x0;
    let xy_flipped;
    if y_diff > x_diff {
        swap(&mut x_diff, &mut y_diff);
        swap(&mut x0, &mut y0);
        swap(&mut x1, &mut y1);
        xy_flipped = true;
    } else {
        xy_flipped = false;
    }
    if x0 > x1 {
        swap(&mut x0, &mut x1);
        swap(&mut y0, &mut y1);
        x_diff *= -1.0;
        y_diff *= -1.0;
    }
    let slope = y_diff / x_diff;
    let y_intercept = (y0 - slope * x0).round() as i32;
    let imp_line = |x: i32, y: i32| {
        (y_diff as i32 * x) - (x_diff as i32 * y) + (x_diff as i32 * y_intercept)
    };
    let mut y = y0 as i32;
    let mut draw_buffer = vec![];
    for x in (x0.round() as i32)..=(x1.round() as i32) {
        let d = imp_line(2*x, y);
        if d > 0 {
            y += 1;
        }
        if xy_flipped {
            draw_buffer.push(ToDraw::new(y,x, *vertex1_color));
        } else {
            draw_buffer.push(ToDraw::new(x, y, *vertex1_color));
        }
    }
    draw_buffer
}
