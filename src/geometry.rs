use core::fmt;

use crate::color::Color;
use crate::math;
use nalgebra as na;

#[derive(Debug)]
pub enum GeoError<'a> {
    NotDiv3(&'a Geometry),
}

pub type Point = na::Vector4<f32>;
pub type Direction = na::Vector4<f32>;
pub type Transform = na::Matrix4<f32>;
pub type Animation = fn(&mut Geometry, secs: f32);

// assumption: in local space, center = (0,0)
#[derive(Clone)]
pub struct Geometry {
    pub vertices: Vec<Vertex>,
    pub vertex_locations: Vec<Point>,
    pub geo_type: GeometryType,
    translation: Transform,
    rotation: Transform,
    scale: Transform,
    animation: Option<Animation>,
    name: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Vertex {
    pub index: usize,
    pub color: Color,
}

#[derive(Debug, Clone, PartialEq, Eq)]
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
            translation: na::Matrix4::identity(),
            scale: na::Matrix4::identity(),
            rotation: na::Matrix4::identity(),
            animation: None,
            name: None,
        }
    }

    pub fn set_name(&mut self, name: Option<String>) {
        self.name = name;
    }

    pub fn transform(&mut self, matrix: Transform) {
        for vertex in &mut self.vertex_locations {
            *vertex = matrix * *vertex;
        }
    }

    pub fn set_animation(&mut self, animation: Animation) {
        self.animation = Some(animation);
    }

    pub fn animate(&mut self, time: f32) {
        if let Some(animation) = self.animation {
            animation(self, time);
        }
    }

    pub fn set_position(&mut self, point: Point) {
        self.translation = math::translation_matrix(point);
    }

    pub fn translate(&mut self, dir: Direction) {
        self.translation *= math::translation_matrix(dir);
    }

    pub fn rotation(&mut self, x_rotation: f32, y_rotation: f32, z_rotation: f32) {
        let mut rotation = na::Matrix4::<f32>::identity();
        let rotations = [
            math::x_rotation_matrix(x_rotation),
            math::y_rotation_matrix(y_rotation),
            math::z_rotation_matrix(z_rotation),
        ];
        for r in rotations {
            rotation *= r;
        }
        self.rotation = rotation;
    }

    pub fn scale(&mut self, scale: na::Vector3<f32>) {
        self.scale = math::scale_matrix(scale);
    }

    pub fn local_to_world(&self, time: f32, cam_rotation: Transform) -> Self {
        let mut copy = self.clone();
        copy.animate(time);
        let transformation_matrix = cam_rotation * copy.translation * copy.rotation * copy.scale;
        copy.transform(transformation_matrix);
        copy
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

    pub fn set_color(&mut self, color: Color) {
        for v in &mut self.vertices {
            v.color = color;
        }
    }
}

impl fmt::Debug for Geometry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name.as_ref().unwrap().as_str())
    }
}

impl Vertex {
    fn new(index: usize, color: Color) -> Self {
        Self { index, color }
    }
}

pub fn point(x: f32, y: f32, z: f32) -> Point {
    na::Vector4::new(x, y, z, 1.0)
}

pub fn direction(x: f32, y: f32, z: f32) -> Direction {
    na::Vector4::new(x, y, z, 0.0)
}

pub fn line() -> Geometry {
    let mut line = Geometry::new(GeometryType::Line);
    line.vertex_locations.push(point(0.0, 0.0, 0.0));
    line.vertex_locations.push(point(1.0, 0.0, 1.0));
    line.vertices.push(Vertex::new(0, Color::Red));
    line.vertices.push(Vertex::new(1, Color::Blue));
    line
}

pub fn triangle() -> Geometry {
    let mut triangle = Geometry::new(GeometryType::Triangle);
    triangle.vertex_locations.push(point(-1.0, -1.0, 0.0));
    triangle.vertex_locations.push(point(0.0, 1.0, 0.0));
    triangle.vertex_locations.push(point(1.0, -1.0, 0.0));
    triangle.vertices.push(Vertex::new(0, Color::Red));
    triangle.vertices.push(Vertex::new(1, Color::Blue));
    triangle.vertices.push(Vertex::new(2, Color::Green));
    triangle
}

pub fn right_triangle() -> Geometry {
    let mut triangle = Geometry::new(GeometryType::Triangle);
    triangle.vertex_locations.push(point(0.0, -1.0, 0.0));
    triangle.vertex_locations.push(point(0.0, 1.0, 0.0));
    triangle.vertex_locations.push(point(1.0, 1.0, 0.0));
    triangle.vertices.push(Vertex::new(0, Color::Red));
    triangle.vertices.push(Vertex::new(1, Color::Blue));
    triangle.vertices.push(Vertex::new(2, Color::Green));
    triangle
}

pub fn square() -> Geometry {
    let mut square = Geometry::new(GeometryType::Triangle);
    square.vertex_locations.push(point(-1.0, -1.0, 0.0));
    square.vertex_locations.push(point(-1.0, 1.0, 0.0));
    square.vertex_locations.push(point(1.0, 1.0, 0.0));
    square.vertex_locations.push(point(1.0, -1.0, 0.0));
    square.vertices.push(Vertex::new(0, Color::Blue));
    square.vertices.push(Vertex::new(1, Color::White));
    square.vertices.push(Vertex::new(2, Color::Red));
    square.vertices.push(Vertex::new(1, Color::White));
    square.vertices.push(Vertex::new(3, Color::Green));
    square.vertices.push(Vertex::new(2, Color::Red));
    square
}
