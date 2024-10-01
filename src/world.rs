use crate::geometry::{direction, point, Geometry, Point};
use crate::math::translation_matrix;

#[derive(Default)]
pub struct World {
    objects: Vec<Geometry>,
}

pub struct Camera {
    x: f32,
    y: f32,
    height: f32,
    width: f32,
}

impl World {
    pub fn insert(&mut self, obj: Geometry) {
        self.objects.push(obj);
    }
}

impl Camera {
    pub fn new(width: f32, height: f32) -> Self {
        // to-do: rethink this
        //assert!(width >= 1.0 && height >= 1.0);
        Self {
            x: 0.0,
            y: 0.0,
            height,
            width,
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
        self.x += x;
        self.y += y;
    }

    pub fn world_view(
        &self,
        world: &World,
        target_width: f32,
        target_height: f32,
    ) -> Vec<Geometry> {
        let mut in_view: Vec<Geometry> = world
            .objects
            .iter()
            .map(|x| x.local_to_world())
            .filter(|x| self.obj_view(x))
            .collect();
        let translation = translation_matrix(direction(-self.x, -self.y, 0.0));
        for obj in &mut in_view {
            obj.transform(translation);
            // to-do: update in 3d
            obj.camera_to_screen(self.width, self.height, target_width, target_height);
        }
        in_view
    }

    fn obj_view(&self, obj: &Geometry) -> bool {
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
