use nalgebra as na;

pub fn f32_compare(left: f32, right: f32) -> bool {
    let epsilon = 0.00001;
    (left - right).abs() < epsilon
}

pub fn translation_matrix(vec: na::Vector3<f32>) -> na::Matrix4<f32> {
    na::matrix![1.0,0.0,0.0,vec.x;
                0.0,1.0,0.0,vec.y;
                0.0,0.0,1.0,vec.z;
                0.0,0.0,0.0,1.0]
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

    #[test]
    fn test_f32_compare() {
        assert!(f32_compare(0.000001, 0.0));
        assert!(f32_compare(0.0, 0.000001));
        assert!(f32_compare(-0.000001, 0.0));
        assert!(f32_compare(0.0, -0.000001));
        assert!(f32_compare(-0.000001, -0.00000001));
        assert!(f32_compare(-0.00000001, -0.000001));
        assert!(!f32_compare(0.1, 0.0));
        assert!(!f32_compare(0.0, 0.1));
    }
}
