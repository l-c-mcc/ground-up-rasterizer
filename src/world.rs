use crate::geometry::{point, Geometry, Point};

#[derive(Default)]
pub struct World {
    objects: Vec<Geometry>,
}

pub struct Camera {
    x: i32,
    y: i32,
    height: i32,
    width: i32,
}

impl World {
    pub fn insert(&mut self, obj: Geometry) {
        self.objects.push(obj);
    }
}

impl Camera {
    pub fn new(width: i32, height: i32) -> Self {
        assert!(width >= 1 && height >= 1);
        Self {
            x: 0,
            y: 0,
            height,
            width,
        }
    }

    pub fn position(&self) -> Point {
        point(self.x as f32, self.y as f32, 0.0)
    }

    pub fn reposition(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }

    pub fn world_view<'a>(&self, world: &'a World) -> Vec<&'a Geometry> {
        world.objects.iter().filter(|x| self.obj_view(x)).collect()
    }

    fn obj_view(&self, obj: &Geometry) -> bool {
        let in_bounds = |x, y| {
            let x_range = self.x as f32..(self.x + self.width) as f32;
            let y_range = self.y as f32..(self.y + self.height) as f32;
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
