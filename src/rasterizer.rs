use crate::color::Rgba;
use crate::geometry::{Geometry, GeometryType, GeoError};
use nalgebra as na;
use std::mem::swap;

pub struct ToDraw {
    pub x: i32,
    pub y: i32,
    pub color: Rgba,
    _depth: f32,
}

impl ToDraw {
    fn new(x: i32, y: i32, color: Rgba) -> Self {
        Self {
            x,
            y,
            color,
            _depth: 0.0,
        }
    }
}

// to-do: handle depth
// to-do: switch geomoetry from vec to one obj
pub fn rasterize_geometry(geometry: &Vec<Geometry>) -> Result<Vec<ToDraw>, GeoError> {
    let mut draw_buffer = vec![];
    for obj in geometry {
        match obj.geo_type {
            GeometryType::Line => {
                let len = geometry.len();
                for i in 0..len {
                    let v1 = &obj.vertices[i];
                    let v2 = &obj.vertices[i + 1];
                    draw_buffer.append(&mut draw_line(
                        &obj.vertex_locations[v1.index],
                        &obj.vertex_locations[v2.index],
                        &(&v1.color).into(),
                        &(&v2.color).into(),
                    ));
                }
            }
            GeometryType::Triangle => {
                let len = obj.vertices.len();
                if len % 3 != 0 {
                    return Err(GeoError::NotDiv3(obj.clone()));
                }
                let mut i = 0;
                while i < len {
                    let v1 = &obj.vertices[i];
                    let v2 = &obj.vertices[i+1];
                    let v3 = &obj.vertices[i+2];
                    draw_buffer.append(&mut rasterize_triangle(
                        &obj.vertex_locations[v1.index], 
                        &obj.vertex_locations[v2.index],
                        &obj.vertex_locations[v3.index], 
                        &(&v1.color).into(),
                        &(&v2.color).into(),
                        &(&v3.color).into()
                    ));
                    i += 3;
                }
            },
        }
    }
    Ok(draw_buffer)
}

/// Implementation of Bresenham's line drawing algorithm.
/// Takes two points and returns a ToDraw vector mapping the corresponding line
/// to pixel values.
/// to-do: color params do not need to be refs
fn draw_line(v1: &na::Vector4<f32>, v2: &na::Vector4<f32>, v1c: &Rgba, v2c: &Rgba) -> Vec<ToDraw> {
    // Prepare vars
    let mut v1c = v1c;
    let mut v2c = v2c;
    let mut x0 = v1[0];
    let mut y0 = v1[1];
    let mut x1 = v2[0];
    let mut y1 = v2[1];
    let mut y_diff = y1 - y0;
    let mut x_diff = x1 - x0;
    // Get the line to the point where it has a slope between [0,1]
    // drawn in the positive x direction.
    let xy_flipped = if y_diff.abs() > x_diff.abs() {
        swap(&mut x_diff, &mut y_diff);
        swap(&mut x0, &mut y0);
        swap(&mut x1, &mut y1);
        true
    } else {
        false
    };
    if x0 > x1 {
        swap(&mut x0, &mut x1);
        swap(&mut y0, &mut y1);
        swap(&mut v1c, &mut v2c);
        x_diff *= -1.0;
        y_diff *= -1.0;
    }
    // Set up color eq.
    let rgba_diff = Rgba::color_a(v2c.r - v1c.r, v2c.g - v1c.g, v2c.b - v1c.b, v2c.a - v1c.a);
    let calc_rgba =
        |x, channel0, channel_diff| channel0 + channel_diff * ((x as f32 - x0) / x_diff);
    // update floats to ints
    let mut y_diff = y_diff as i32;
    let x_diff = x_diff as i32;
    let mut y = y0 as i32;
    let y_incr = if y_diff > 0 {
        1
    } else {
        y_diff *= -1;
        -1
    };
    let mut d = x_diff - 2 * y_diff;
    let mut draw_buffer = vec![];
    for x in (x0.round() as i32)..=(x1.round() as i32) {
        let color = Rgba::color_a(
            calc_rgba(x, v1c.r, rgba_diff.r),
            calc_rgba(x, v1c.g, rgba_diff.g),
            calc_rgba(x, v1c.b, rgba_diff.b),
            calc_rgba(x, v1c.a, rgba_diff.a),
        );
        if d > 0 {
            d -= 2 * y_diff;
        } else {
            y += y_incr;
            d += 2 * (x_diff - y_diff);
        }
        if xy_flipped {
            draw_buffer.push(ToDraw::new(y, x, color));
        } else {
            draw_buffer.push(ToDraw::new(x, y, color));
        }
    }
    draw_buffer
}

fn rasterize_triangle(v1: &na::Vector4<f32>, v2: &na::Vector4<f32>, v3: &na::Vector4<f32>, 
                    v1c: &Rgba, v2c: &Rgba, v3c: &Rgba) -> Vec<ToDraw> {
    let x0 = v1[0];
    let x1 = v2[0];
    let x2 = v3[0];
    let y0 = v1[1];
    let y1 = v2[1];
    let y2 = v3[1];
    let f12 = |x, y| {
        (y1-y2) * x + (x2 - x1) * y + x1 * y2 - x2 * y1
    };
    let f20 = |x,y| {
        (y2-y0) * x + (x0 -x2) * y + x2 * y0 - x0 * y2
    };
    let f01 = |x, y| {
        (y0-y1) * x + (x1-x0) * y + x0 * y1 - x1 * y0
    };
    let alpha_denom = f12(x0,y0);
    let beta_denom = f20(x1,y1);
    let lambda_denom = f01(x2,y2);
    let alpha = |x, y| f12(x,y) / alpha_denom;
    let beta = |x, y| f20(x,y) / beta_denom;
    let lambda = |x,y| f01(x,y) / lambda_denom;
    let x_min = x0.min(x1).min(x2).round() as usize;
    let x_max = x0.max(x1).max(x2).round() as usize;
    let y_min = y0.min(y1).min(y2).round() as usize;
    let y_max = y0.max(y1).max(y2).round() as usize;
    let within_bounds = |val| val >= 0.0 && val <= 1.0;
    let mut draw_buffer = vec![];
    for y in (y_min..=y_max).map(|y| y as f32) {
        for x in (x_min..=x_max).map(|x| x as f32) {
            let a = alpha(x,y);
            let b = beta(x,y);
            let l = lambda(x,y);
            if within_bounds(a) && within_bounds(b) && within_bounds(l) {
                draw_buffer.push(ToDraw::new(x as i32, y as i32, &(&(a * v1c) + &(b * v2c)) + &(l * v3c)));
            }
        }
    }
    draw_buffer
}