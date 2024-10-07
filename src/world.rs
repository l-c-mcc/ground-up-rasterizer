use crate::geometry::{direction, point, Geometry, GeometryType, Point};
use crate::math::{self, translation_matrix, z_rotation_matrix};

#[derive(Default)]
pub struct World {
    objects: Vec<Geometry>,
}

pub struct Camera {
    x: f32,
    y: f32,
    height: f32,
    width: f32,
    angle: f32,
}

impl World {
    pub fn insert(&mut self, obj: Geometry) {
        self.objects.push(obj);
    }
}

impl Camera {
    pub fn new(width: f32, height: f32, angle: f32) -> Self {
        // to-do: rethink this
        //assert!(width >= 1.0 && height >= 1.0);
        Self {
            x: 0.0,
            y: 0.0,
            height,
            width,
            angle,
        }
    }

    pub fn position(&self) -> Point {
        point(self.x, self.y, 0.0)
    }

    pub fn reposition(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
    }

    pub fn translate(&mut self, x: f32, y: f32) {
        let rotation_matrix = math::z_rotation_matrix(self.angle);
        let change = rotation_matrix * direction(x, y, 0.0);
        self.x += change.x;
        self.y += change.y;
    }

    pub fn world_view(
        &self,
        world: &World,
        target_width: f32,
        target_height: f32,
        time: f32,
    ) -> Vec<Geometry> {
        let mut in_view: Vec<Geometry> = world
            .objects
            .iter()
            .map(|x| x.local_to_world(time))
            .filter(|x| self.obj_view(x))
            .collect();
        let direction_to_center = direction(
            -(self.x + (self.width / 2.0)),
            -(self.y + (self.height / 2.0)),
            0.0,
        );
        let cam_center_translation = translation_matrix(direction_to_center);
        let rotation = z_rotation_matrix(self.angle);
        let undo_cam_center_translation = translation_matrix(-direction_to_center);
        let translation_to_center = translation_matrix(direction(-self.x, -self.y, 0.0));
        let final_transform =
            translation_to_center * undo_cam_center_translation * rotation * cam_center_translation;
        for obj in &mut in_view {
            obj.transform(final_transform);
            // to-do: update in 3d
            obj.camera_to_screen(self.width, self.height, target_width, target_height);
        }
        in_view
    }

    fn obj_view(&self, obj: &Geometry) -> bool {
        fn intercept(slope: f32, y_int: f32, y: f32, bound_min: f32, bound_max: f32) -> bool {
            let x_input = (y - y_int) / slope;
            x_input >= bound_min && x_input < bound_max
        }
        if self.vertex_in_bounds(obj) {
            return true;
        }
        let vertex_count = obj.vertices.len();
        for i in 0..vertex_count {
            if i == vertex_count && obj.geo_type == GeometryType::Line {
                break;
            }
            let j = if i == vertex_count - 1 { 0 } else { i + 1 };
            let vertex1 = obj.vertex_locations[i];
            let vertex2 = obj.vertex_locations[j];
            let slope = (vertex2.y - vertex1.y) / (vertex2.x - vertex1.x);
            // line is horizontal
            if slope == 0.0 {
                let y_int = vertex1.y;
                if (self.y..=self.y + self.height).contains(&y_int) {
                    return true;
                }
            // line is vertical
            } else if slope.is_infinite() {
                let x_int = vertex1.x;
                if (self.x..=self.x + self.width).contains(&x_int) {
                    return true;
                }
            // other types of lines
            } else {
                let y_int = vertex1.y - slope * vertex1.x;
                let slope_flipped = 1.0 / slope;
                let y_int_flipped = vertex1.x - slope_flipped * vertex1.y;
                if intercept(slope, y_int, self.y, self.x, self.x + self.width)
                    || intercept(
                        slope,
                        y_int,
                        self.y + self.height,
                        self.x,
                        self.x + self.width,
                    )
                    || intercept(
                        slope_flipped,
                        y_int_flipped,
                        self.x,
                        self.y,
                        self.y + self.height,
                    )
                    || intercept(
                        slope_flipped,
                        y_int_flipped,
                        self.x + self.width,
                        self.y,
                        self.y + self.height,
                    )
                {
                    return true;
                }
            }
        }
        false
    }

    fn vertex_in_bounds(&self, obj: &Geometry) -> bool {
        let in_bounds = |x, y| {
            let x_range = self.x..self.x + self.width;
            let y_range = self.y..self.y + self.height;
            x_range.contains(&x) && y_range.contains(&y)
        };
        for vec in &obj.vertex_locations {
            if in_bounds(vec.x, vec.y) {
                return true;
            }
        }
        false
    }
}
