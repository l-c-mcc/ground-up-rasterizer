use crate::color::Color;
use crate::geometry::{Geometry, GeometryType};
use nalgebra as na;
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
pub fn rasterize_geometry(geometry: &Vec<Geometry>) -> Vec<ToDraw> {
    let mut draw_buffer = vec![];
    for obj in geometry {
        match obj.geo_type {
            GeometryType::Line => {
                let len = geometry.len();
                for i in 0..len {
                    let v1 = &obj.vertices[i];
                    let v2 = &obj.vertices[i + 1];
                    draw_buffer.extend(draw_line(
                        &obj.vertex_locations[v1.index],
                        &obj.vertex_locations[v2.index],
                        &v1.color,
                        &v2.color,
                    ));
                }
            }
            GeometryType::Triangle => todo!(),
        }
    }
    draw_buffer
}

/// Uses Bresenham's; look into Wu's for anti-aliasing
/// To-do: Turn into Result?
/// To-do: color interp
fn draw_line(
    v1: &na::Vector4<f32>,
    v2: &na::Vector4<f32>,
    v1c: &Color,
    v2c: &Color,
) -> Vec<ToDraw> {
    let mut vertex1_color = v1c;
    let mut vertex2_color = v2c;
    let mut x0 = v1[0];
    let mut y0 = v1[1];
    let mut x1 = v2[0];
    let mut y1 = v2[1];
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
    let imp_line =
        |x: i32, y: i32| (y_diff as i32 * x) - (x_diff as i32 * y) + (x_diff as i32 * y_intercept);
    let mut y = y0 as i32;
    let mut draw_buffer = vec![];
    for x in (x0.round() as i32)..=(x1.round() as i32) {
        let d = imp_line(2 * x, y);
        if d > 0 {
            y += 1;
        }
        if xy_flipped {
            draw_buffer.push(ToDraw::new(y, x, *vertex1_color));
        } else {
            draw_buffer.push(ToDraw::new(x, y, *vertex1_color));
        }
    }
    draw_buffer
}
