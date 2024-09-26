use crate::color::Color;
use nalgebra as na;

#[derive(Debug)]
pub enum GeoError {
    NotDiv3(Geometry),
    NoVertices(Geometry),
}

pub type Point = na::Vector4<f32>;
pub type Direction = na::Vector4<f32>;

pub fn point(x: f32, y: f32, z: f32) -> Point {
    na::Vector4::new(x, y, z, 1.0)
}

pub fn direction(x: f32, y: f32, z: f32) -> Direction {
    na::Vector4::new(x, y, z, 0.0)
}

pub fn line(t: f32) -> Geometry {
    let mut line = Geometry::new(GeometryType::Line);
    let x = (300.0 * t.cos()) + 500.0;
    let y = (300.0 * t.sin()) + 500.0;
    line.vertex_locations.push(point(500.0, 500.0, 0.0));
    line.vertex_locations.push(point(x, y, 0.0));
    line.vertices.push(Vertex::new(0, Color::Red));
    line.vertices.push(Vertex::new(1, Color::Blue));
    line
}

pub fn triangle() -> Geometry {
    let mut triangle = Geometry::new(GeometryType::Triangle);
    triangle.vertex_locations.push(point(450.0, 400.0, 0.0));
    triangle.vertex_locations.push(point(600.0, 600.0, 0.0));
    triangle.vertex_locations.push(point(300.0, 600.0, 0.0));
    triangle.vertices.push(Vertex::new(0, Color::Red));
    triangle.vertices.push(Vertex::new(1, Color::Blue));
    triangle.vertices.push(Vertex::new(2, Color::Green));
    triangle.center().unwrap();
    triangle
}

pub fn square(scale: f32) -> Geometry {
    let mut square = Geometry::new(GeometryType::Triangle);
    square.vertex_locations.push(scale * point(0.0, 0.0, 0.0));
    square.vertex_locations.push(scale * point(1.0, 0.0, 0.0));
    square.vertex_locations.push(scale * point(0.0, 1.0, 0.0));
    square.vertex_locations.push(scale * point(1.0, 1.0, 0.0));
    square.vertices.push(Vertex::new(0, Color::Blue));
    square.vertices.push(Vertex::new(1, Color::White));
    square.vertices.push(Vertex::new(2, Color::Red));
    square.vertices.push(Vertex::new(1, Color::White));
    square.vertices.push(Vertex::new(3, Color::Green));
    square.vertices.push(Vertex::new(2, Color::Red));
    square
}

#[derive(Debug, Clone)]
pub struct Geometry {
    pub vertices: Vec<Vertex>,
    pub vertex_locations: Vec<Point>,
    pub geo_type: GeometryType,
    center: Option<Point>,
}

#[derive(Debug, Clone)]
pub struct Vertex {
    pub index: usize,
    pub color: Color,
}

#[derive(Debug, Clone)]
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
            center: None,
        }
    }

    pub fn transform(&mut self, matrix: na::Matrix4<f32>) {
        for vertex in &mut self.vertex_locations {
            *vertex = matrix * *vertex;
        }
        if let Some(c) = self.center {
            self.center = Some(matrix * c);
        }
    }

    fn center(&mut self) -> Result<(), GeoError> {
        if let Some(first_v) = self.vertex_locations.first() {
            let mut min = *first_v;
            let mut max = min;
            for v in &self.vertex_locations {
                for i in 0..=2 {
                    if v[i] < min[i] {
                        min[i] = v[i];
                    } else if v[i] > max[i] {
                        max[i] = v[i];
                    }
                }
            }
            self.center = Some((max + min) / 2.0);
            Ok(())
        } else {
            Err(GeoError::NoVertices(self.clone()))
        }
    }

    // to-do: make 4d direction vec
    pub fn vec_from_origin(&self) -> Option<na::Vector3<f32>> {
        self.center.map(|c| na::Vector3::new(c.x, c.y, c.z))
    }

    pub fn camera_to_screen(
        &mut self,
        camera_width: f32,
        camera_height: f32,
        screen_width: f32,
        screen_height: f32,
    ) {
        for vertex in &mut self.vertex_locations {
            let x_ratio = vertex.x / camera_width;
            let y_ratio = vertex.y / camera_height;
            vertex.x = x_ratio * screen_width;
            vertex.y = y_ratio * screen_height;
        }
    }
}

impl Vertex {
    fn new(index: usize, color: Color) -> Self {
        Self { index, color }
    }
}
