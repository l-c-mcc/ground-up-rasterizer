use crate::geometry::{Geometry, GeometryType};

pub struct ToDraw {

}

// to-do: handle depth
pub fn rasterize_geometry(geometry: Vec<Geometry>) {
    for obj in geometry {
        match obj.geo_type {
            GeometryType::Line => todo!(),
            GeometryType::Triangle => todo!(),
        }
    }
}