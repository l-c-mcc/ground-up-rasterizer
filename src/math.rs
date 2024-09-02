pub fn f32_compare(left: f32, right: f32) -> bool {
    let epsilon = 0.00001;
    if (left - right).abs() < epsilon {
        true
    } else {
        false
    }
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