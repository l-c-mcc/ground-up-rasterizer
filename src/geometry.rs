use nalgebra as na;
use crate::color::Color;

pub fn line() -> Geometry {
    let mut line = Geometry::new(GeometryType::Line);
    line.vertex_locations.push(na::Vector4::new(0.0, 0.0, 0.0, 1.0));
    line.vertex_locations.push(na::Vector4::new(10.0, 10.0, 0.0, 1.0));
    line.vertices.push(Vertex::new(0, Color::Red));
    line.vertices.push(Vertex::new(1, Color::Red));
    line
}

/*
pub fn cube(c: Color) -> Geometry {
    let mut cube = Geometry::new(GeometryType::Triangle);
    cube.vertex_locations.push(na::Vector4::new(0.0, 0.0, 0.0, 1.0));
    cube
}
    */

struct Geometry {
    vertices: Vec<Vertex>,
    vertex_locations: Vec<na::Vector4<f32>>,
    geo_type: GeometryType,
}

struct Vertex {
    index: usize,
    color: Color,
}

enum GeometryType {
    Triangle,
    Line,
}

impl Geometry {
    fn new(geo_type: GeometryType) -> Self {
        Self {
            vertices: vec![],
            vertex_locations: vec![],
            geo_type
        }
    }
}

impl Vertex {
    fn new(index: usize, color: Color) -> Self {
        Self {
            index,
            color,
        }
    }
}