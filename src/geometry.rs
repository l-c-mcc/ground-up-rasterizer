use core::{fmt, panic};

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

    pub fn transform_proj(&mut self, matrix: Transform) {
        for vertex in &mut self.vertex_locations {
            let z = vertex.z;
            *vertex = matrix * *vertex;
            // to-do: camera view box
            if vertex.w == 0.0 {
                //*vertex /= 0.001;
            } else {
                *vertex /= vertex.w;
            }
            vertex.z = z;
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

    pub fn local_to_world(&self, time: f32, cam_rotation: Transform, dis: f32) -> Self {
        let mut copy = self.clone();

        copy.animate(time);
        let transformation_matrix = cam_rotation * copy.translation * copy.rotation * copy.scale;
        copy.transform(transformation_matrix);
        // very hacky way to stop triangles behind camera from being included.
        // This could be much better!
        // to-do: update to support lines
        if copy.geo_type == GeometryType::Triangle {
            let get_z = |i| {
                let vertex: &Vertex = &self.vertices[i];
                let index = (*vertex).index;
                copy.vertex_locations[index].z
            };
            let lt_dis = |i| get_z(i) < -dis;
            copy.vertices.clear();
            for i in (0..self.vertices.len()).step_by(3) {
                if lt_dis(i) && lt_dis(i + 1) && lt_dis(i + 2) {
                    copy.vertices.push(self.vertices[i].clone());
                    copy.vertices.push(self.vertices[i + 1].clone());
                    copy.vertices.push(self.vertices[i + 2].clone());
                }
            }
        }

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

pub fn cube() -> Geometry {
    let mut cube = Geometry::new(GeometryType::Triangle);
    cube.vertex_locations.push(point(-1.0, -1.0, 1.0));
    cube.vertex_locations.push(point(-1.0, -1.0, -1.0));
    cube.vertex_locations.push(point(-1.0, 1.0, -1.0));
    cube.vertex_locations.push(point(-1.0, 1.0, 1.0));
    cube.vertex_locations.push(point(1.0, -1.0, -1.0));
    cube.vertex_locations.push(point(1.0, 1.0, -1.0));
    cube.vertex_locations.push(point(1.0, 1.0, 1.0));
    cube.vertex_locations.push(point(1.0, -1.0, 1.0));
    // to-do: may need to reorder if back face culling is implemented
    // front face
    cube.vertices.push(Vertex::new(0, Color::Blue));
    cube.vertices.push(Vertex::new(3, Color::Blue));
    cube.vertices.push(Vertex::new(6, Color::Blue));
    cube.vertices.push(Vertex::new(6, Color::Blue));
    cube.vertices.push(Vertex::new(7, Color::Blue));
    cube.vertices.push(Vertex::new(0, Color::Blue));
    // left face
    cube.vertices.push(Vertex::new(0, Color::Yellow));
    cube.vertices.push(Vertex::new(3, Color::Yellow));
    cube.vertices.push(Vertex::new(2, Color::Yellow));
    cube.vertices.push(Vertex::new(2, Color::Yellow));
    cube.vertices.push(Vertex::new(1, Color::Yellow));
    cube.vertices.push(Vertex::new(0, Color::Yellow));
    // right face
    cube.vertices.push(Vertex::new(6, Color::Red));
    cube.vertices.push(Vertex::new(7, Color::Red));
    cube.vertices.push(Vertex::new(4, Color::Red));
    cube.vertices.push(Vertex::new(4, Color::Red));
    cube.vertices.push(Vertex::new(5, Color::Red));
    cube.vertices.push(Vertex::new(6, Color::Red));
    // back face
    cube.vertices.push(Vertex::new(5, Color::Cyan));
    cube.vertices.push(Vertex::new(4, Color::Cyan));
    cube.vertices.push(Vertex::new(1, Color::Cyan));
    cube.vertices.push(Vertex::new(1, Color::Cyan));
    cube.vertices.push(Vertex::new(2, Color::Cyan));
    cube.vertices.push(Vertex::new(5, Color::Cyan));
    // top face
    cube.vertices.push(Vertex::new(0, Color::Magenta));
    cube.vertices.push(Vertex::new(1, Color::Magenta));
    cube.vertices.push(Vertex::new(4, Color::Magenta));
    cube.vertices.push(Vertex::new(4, Color::Magenta));
    cube.vertices.push(Vertex::new(7, Color::Magenta));
    cube.vertices.push(Vertex::new(0, Color::Magenta));
    // bottom face
    cube.vertices.push(Vertex::new(2, Color::Green));
    cube.vertices.push(Vertex::new(3, Color::Green));
    cube.vertices.push(Vertex::new(6, Color::Green));
    cube.vertices.push(Vertex::new(6, Color::Green));
    cube.vertices.push(Vertex::new(5, Color::Green));
    cube.vertices.push(Vertex::new(2, Color::Green));
    cube
}

#[cfg(test)]
mod test {
    use math::projection_matrix;

    use super::*;

    #[test]
    //to-do: better name
    fn test_projection_transform() {
        let vec = na::vector![1.0, 1.0, 1.0, 1.0];
        let d = 10.0;
        let proj = projection_matrix(0.0, 0.0, d, None);
        let result = proj * vec;
        let target: na::Vector4<f32> = na::vector![1.0, 1.0, 1.0, -0.1];
        assert_eq!(result, target);
        let norm_target = na::vector![-10.0, -10.0, -10.0, 1.0];
        let result = result / result.w;
        assert_eq!(result, norm_target);
    }
}
