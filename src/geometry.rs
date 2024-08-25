use crate::color::Color;
use nalgebra as na;

pub fn line() -> Geometry {
    let mut line = Geometry::new(GeometryType::Line);
    line.vertex_locations
        .push(na::Vector4::new(0.0, 0.0, 0.0, 1.0));
    line.vertex_locations
        .push(na::Vector4::new(999.0, 999.0, 0.0, 1.0));
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

pub struct Geometry {
    pub vertices: Vec<Vertex>,
    pub vertex_locations: Vec<na::Vector4<f32>>,
    pub geo_type: GeometryType,
}

pub struct Vertex {
    pub index: usize,
    pub color: Color,
}

pub enum GeometryType {
    Triangle,
    Line,
}

impl Geometry {
    fn new(geo_type: GeometryType) -> Self {
        Self {
            vertices: vec![],
            vertex_locations: vec![],
            geo_type,
        }
    }
}

impl Vertex {
    fn new(index: usize, color: Color) -> Self {
        Self { index, color }
    }
}
