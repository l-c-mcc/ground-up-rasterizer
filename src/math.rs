use nalgebra as na;
use std::cmp;
use std::ops::{Add, Mul};

#[derive(Debug, Clone, Copy)]
pub struct OrdFloat(pub f32);

impl Add for OrdFloat {
    type Output = OrdFloat;
    fn add(self, rhs: Self) -> Self::Output {
        OrdFloat(self.0 + rhs.0)
    }
}

impl Mul for OrdFloat {
    type Output = OrdFloat;
    fn mul(self, rhs: Self) -> Self::Output {
        OrdFloat(self.0 * rhs.0)
    }
}

impl PartialEq for OrdFloat {
    fn eq(&self, other: &Self) -> bool {
        f32_equals(self.0, other.0)
    }
}

impl Eq for OrdFloat {}

impl PartialOrd for OrdFloat {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for OrdFloat {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        f32_compare(self.0, other.0).unwrap()
    }
}

impl From<f32> for OrdFloat {
    fn from(value: f32) -> Self {
        OrdFloat(value)
    }
}

impl From<OrdFloat> for f32 {
    fn from(value: OrdFloat) -> Self {
        value.0
    }
}

pub fn f32_equals(left: f32, right: f32) -> bool {
    // to-do: worry about NaN case
    Some(cmp::Ordering::Equal) == f32_compare(left, right)
}

pub fn f32_compare(left: f32, right: f32) -> Option<cmp::Ordering> {
    let epsilon = 0.00001;
    if (left - right).abs() < epsilon {
        Some(cmp::Ordering::Equal)
    } else {
        left.partial_cmp(&right)
    }
}

pub fn translation_matrix(vec: na::Vector3<f32>) -> na::Matrix4<f32> {
    na::matrix![1.0,0.0,0.0,vec.x;
                0.0,1.0,0.0,vec.y;
                0.0,0.0,1.0,vec.z;
                0.0,0.0,0.0,1.0]
}

pub fn x_rotation_matrix(theta: f32) -> na::Matrix4<f32> {
    na::matrix![1.0, 0.0, 0.0, 0.0;
                0.0, theta.cos(), -(theta.sin()), 0.0;
                0.0, theta.sin(), theta.cos(), 0.0;
                0.0, 0.0, 0.0, 1.0]
}

pub fn y_rotation_matrix(theta: f32) -> na::Matrix4<f32> {
    na::matrix![theta.cos(), 0.0, theta.sin(), 0.0;
                0.0, 1.0, 0.0, 0.0;
                -(theta.sin()), 0.0, theta.cos(), 0.0;
                0.0, 0.0, 0.0, 1.0]
}

pub fn z_rotation_matrix(theta: f32) -> na::Matrix4<f32> {
    na::matrix![theta.cos(), -(theta.sin()), 0.0, 0.0;
                theta.sin(), theta.cos(), 0.0, 0.0;
                0.0, 0.0, 1.0, 0.0;
                0.0, 0.0, 0.0, 1.0]
}

pub fn scale_matrix(vec: na::Vector3<f32>) -> na::Matrix4<f32> {
    na::matrix![vec.x, 0.0, 0.0, 0.0;
                0.0, vec.y, 0.0, 0.0;
                0.0, 0.0, vec.z, 0.0;
                0.0, 0.0, 0.0, 1.0]
}

#[cfg(test)]
mod tests {
    use super::*;

    // to-do: update
    #[test]
    fn test_f32_comparison() {
        assert_eq!(f32_compare(0.000001, 0.0), Some(cmp::Ordering::Equal));
        assert_eq!(f32_compare(0.0, 0.000001), Some(cmp::Ordering::Equal));
        assert_eq!(f32_compare(-0.000001, 0.0), Some(cmp::Ordering::Equal));
        assert_eq!(f32_compare(0.0, -0.000001), Some(cmp::Ordering::Equal));
        assert_eq!(
            f32_compare(-0.000001, -0.00000001),
            Some(cmp::Ordering::Equal)
        );
        assert_eq!(
            f32_compare(-0.00000001, -0.000001),
            Some(cmp::Ordering::Equal)
        );
        assert_eq!(f32_compare(0.1, 0.0), Some(cmp::Ordering::Greater));
        assert_eq!(f32_compare(0.0, 0.1), Some(cmp::Ordering::Less));
    }
}
