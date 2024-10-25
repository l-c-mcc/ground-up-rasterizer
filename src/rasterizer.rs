use crate::color::Rgba;
use crate::geometry::{GeoError, Geometry, GeometryType, Point};
use crate::math::OrdFloat;
use std::mem::swap;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ToDraw {
    pub x: i32,
    pub y: i32,
    pub color: Rgba,
    pub depth: OrdFloat,
}

impl ToDraw {
    fn new(x: i32, y: i32, color: Rgba, depth: f32) -> Self {
        Self {
            x,
            y,
            color,
            depth: OrdFloat(depth),
        }
    }
}

pub fn rasterize_geometry<'a>(geometry: &'a Geometry, draw_buffer: &mut Vec<ToDraw>) -> Result<(), GeoError<'a>> {
    match geometry.geo_type {
        GeometryType::Line => {
            let len = geometry.vertices.len();
            for i in 0..len - 1 {
                let v1 = &geometry.vertices[i];
                let v2 = &geometry.vertices[i + 1];
                draw_line(
                    &geometry.vertex_locations[v1.index],
                    &geometry.vertex_locations[v2.index],
                    &(&v1.color).into(),
                    &(&v2.color).into(),
                    draw_buffer
                );
            }
        }
        GeometryType::Triangle => {
            let len = geometry.vertices.len();
            if len % 3 != 0 {
                return Err(GeoError::NotDiv3(geometry));
            }
            let mut i = 0;
            while i < len {
                // move this inside Geometry struct?
                let v1 = &geometry.vertices[i];
                let v2 = &geometry.vertices[i + 1];
                let v3 = &geometry.vertices[i + 2];
                rasterize_triangle(
                    &geometry.vertex_locations[v1.index],
                    &geometry.vertex_locations[v2.index],
                    &geometry.vertex_locations[v3.index],
                    &(&v1.color).into(),
                    &(&v2.color).into(),
                    &(&v3.color).into(),
                    draw_buffer
                );
                i += 3;
            }
        }
    }
    Ok(())
}

/// Implementation of Bresenham's line drawing algorithm.
/// Takes two points and returns a ToDraw vector mapping the corresponding line
/// to pixel values.
/// to-do: color params do not need to be refs
fn draw_line(v1: &Point, v2: &Point, v1c: &Rgba, v2c: &Rgba, draw_buffer: &mut Vec<ToDraw>) {
    // Prepare vars
    let mut v1c = v1c;
    let mut v2c = v2c;
    let mut x0 = v1.x;
    let mut y0 = v1.y;
    let mut z0 = v1.z;
    let mut x1 = v2.x;
    let mut y1 = v2.y;
    let mut z1 = v2.z;
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
        swap(&mut z0, &mut z1);
        x_diff *= -1.0;
        y_diff *= -1.0;
    }
    // Set up color and depth eq.
    let rgba_diff = v2c - v1c;
    let depth_diff = OrdFloat(z1 - z0);
    let linear_interp = |x: i32, initial: OrdFloat, final_minus_initial: OrdFloat| {
        (initial + final_minus_initial * OrdFloat((x as f32 - x0) / x_diff)).0
    };
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
    let d_incr_gte_0 = -2 * y_diff;
    let d_incr_lt_0 = 2 * (x_diff - y_diff);
    let x0 = x0.round() as i32;
    let y0 = y0.round() as i32;
    if xy_flipped {
        draw_buffer.push(ToDraw::new(y0, x0, v1c.clone(), z0));
    } else {
        draw_buffer.push(ToDraw::new(x0, y0, v1c.clone(), z0));
    }
    for x in (x0 + 1)..=(x1.round() as i32) {
        let color = Rgba::color_a(
            linear_interp(x, v1c.r, rgba_diff.r),
            linear_interp(x, v1c.g, rgba_diff.g),
            linear_interp(x, v1c.b, rgba_diff.b),
            linear_interp(x, v1c.a, rgba_diff.a),
        );
        let depth = linear_interp(x, OrdFloat(z0), depth_diff);
        if d >= 0 {
            d += d_incr_gte_0;
        } else {
            y += y_incr;
            d += d_incr_lt_0;
        }
        if xy_flipped {
            draw_buffer.push(ToDraw::new(y, x, color, depth));
        } else {
            draw_buffer.push(ToDraw::new(x, y, color, depth));
        }
    }
}

fn rasterize_triangle(
    v1: &Point,
    v2: &Point,
    v3: &Point,
    v1c: &Rgba,
    v2c: &Rgba,
    v3c: &Rgba,
    draw_buffer: &mut Vec<ToDraw>
) {
    let x0 = v1[0].round();
    let x1 = v2[0].round();
    let x2 = v3[0].round();
    let y0 = v1[1].round();
    let y1 = v2[1].round();
    let y2 = v3[1].round();
    let f12 = |x, y| (y1 - y2) * x + (x2 - x1) * y + x1 * y2 - x2 * y1;
    let f20 = |x, y| (y2 - y0) * x + (x0 - x2) * y + x2 * y0 - x0 * y2;
    let f01 = |x, y| (y0 - y1) * x + (x1 - x0) * y + x0 * y1 - x1 * y0;
    let alpha_denom = f12(x0, y0);
    let beta_denom = f20(x1, y1);
    let lambda_denom = f01(x2, y2);
    let alpha = |x, y| f12(x, y) / alpha_denom;
    let beta = |x, y| f20(x, y) / beta_denom;
    let lambda = |x, y| f01(x, y) / lambda_denom;
    let x_min = x0.min(x1).min(x2) as usize;
    let x_max = x0.max(x1).max(x2) as usize;
    let y_min = y0.min(y1).min(y2) as usize;
    let y_max = y0.max(y1).max(y2) as usize;
    let within_bounds = |val| (0.0..=1.0).contains(&val);
    for y in (y_min..=y_max).map(|y| y as f32) {
        for x in (x_min..=x_max).map(|x| x as f32) {
            let a = alpha(x, y);
            let b = beta(x, y);
            let l = lambda(x, y);
            if within_bounds(a) && within_bounds(b) && within_bounds(l) {
                draw_buffer.push(ToDraw::new(
                    x as i32,
                    y as i32,
                    &(&(a * v1c) + &(b * v2c)) + &(l * v3c),
                    (a * v1.z) + (b * v2.z) + (l * v3.z),
                ));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::color::Color;
    use crate::geometry::point;

    use super::*;
    use std::collections::BTreeSet;

    #[test]
    fn test_point() {
        let x = 1;
        let y = 1;
        let c = Rgba::color(1.0, 0.0, 0.0);
        let target = vec![ToDraw::new(x, y, c.clone(), 0.0)];
        let mut calculated = vec![];
        let vertex = point(x as f32, y as f32, 0.0);
        draw_line(&vertex, &vertex, &c, &c, &mut calculated);
        assert_eq!(calculated, target);
    }

    // to-do: handle depth when in 3d (another test?)
    #[test]
    fn test_line() {
        let x = 1;
        let y = 1;
        let c = Rgba::color(1.0, 0.0, 0.0);
        let origin = ToDraw::new(x, y, c.clone(), 0.0);
        let vertex1: Point = point(x as f32, y as f32, 0.0);
        for x in (-1..=1).map(|x| x as f32) {
            for y in (-1..=1).map(|y| y as f32) {
                if x == 0.0 && y == 0.0 {
                    continue;
                }
                let mut vertex2 = vertex1;
                vertex2.x += x;
                vertex2.y += y;
                let mut line = vec![];
                draw_line(&vertex1, &vertex2, &c, &c, &mut line);
                let target_point = ToDraw::new(
                    (vertex1.x + x) as i32,
                    (vertex1.y + y) as i32,
                    c.clone(),
                    0.0,
                );
                let mut target_line = BTreeSet::new();
                assert!(target_line.insert(&origin));
                assert!(target_line.insert(&target_point));
                let computed_line = BTreeSet::from_iter(line.iter());
                assert_eq!(target_line, computed_line);
            }
        }
    }

    #[test]
    fn test_fp_line() {
        let x0: f32 = 0.8;
        let y0: f32 = 1.1;
        let x1: f32 = 2.4;
        let y1: f32 = 2.3;
        let c: Rgba = (&Color::Red).into();
        let target_line = vec![
            ToDraw::new(x0.round() as i32, y0.round() as i32, c.clone(), 0.0),
            ToDraw::new(x1.round() as i32, y1.round() as i32, c.clone(), 0.0),
        ];
        let mut computed_line = vec![];
        draw_line(&point(x0, y0, 0.0), &point(x1, y1, 0.0), &c, &c, &mut computed_line);
        assert_eq!(
            BTreeSet::from_iter(target_line.into_iter()),
            BTreeSet::from_iter(computed_line.into_iter()),
        );
    }

    #[test]
    fn test_triangle() {
        let color: Rgba = (&Color::Red).into();
        let v1 = point(0.0, 0.0, 0.0);
        let v2 = point(2.0, 0.0, 0.0);
        let v3 = point(1.0, 1.0, 0.0);
        let mut computed_triangle = vec![];
        rasterize_triangle(&v1, &v2, &v3, &color, &color, &color, &mut computed_triangle);
        let target_triangle = vec![
            ToDraw::new(v1.x as i32, v1.y as i32, color.clone(), 0.0),
            ToDraw::new(v2.x as i32, v2.y as i32, color.clone(), 0.0),
            ToDraw::new(v3.x as i32, v3.y as i32, color.clone(), 0.0),
            ToDraw::new(
                (v1.x as i32 + v2.x as i32) / 2,
                v1.y as i32,
                color.clone(),
                0.0,
            ),
        ];
        assert_eq!(
            BTreeSet::from_iter(target_triangle.into_iter()),
            BTreeSet::from_iter(computed_triangle.into_iter())
        );
    }

    #[test]
    fn test_triangle_fp() {
        let color: Rgba = (&Color::Red).into();
        let v1 = point(0.1, 0.2, 0.0);
        let v2 = point(1.8, 0.3, 0.0);
        let v3 = point(1.1, 0.9, 0.0);
        let mut computed_triangle = vec![]; 
        rasterize_triangle(&v1, &v2, &v3, &color, &color, &color, &mut computed_triangle);
        let target_triangle = vec![
            ToDraw::new(v1.x.round() as i32, v1.y.round() as i32, color.clone(), 0.0),
            ToDraw::new(v2.x.round() as i32, v2.y.round() as i32, color.clone(), 0.0),
            ToDraw::new(v3.x.round() as i32, v3.y.round() as i32, color.clone(), 0.0),
            ToDraw::new(
                (v1.x.round() as i32 + v2.x.round() as i32) / 2,
                v1.y.round() as i32,
                color.clone(),
                0.0,
            ),
        ];
        assert_eq!(
            BTreeSet::from_iter(target_triangle.into_iter()),
            BTreeSet::from_iter(computed_triangle.into_iter())
        );
    }
}