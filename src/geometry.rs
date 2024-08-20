use nalgebra as na;
use crate::color::Color;

struct Geometry {
    // (points, color) - vector point type, color
    vertex_indices: Vec<usize>,
    vertex_list: Vec<na::Vector4<f32>>,
    g_type: GeometryType,
}

struct Vertex {
    index: usize,
    color: Color,
}

enum GeometryType {
    Triangle,
    Line,
}