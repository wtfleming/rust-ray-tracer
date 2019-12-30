pub mod matrix;
pub mod ray;
pub mod sphere;
pub mod vector3;
pub mod vector4;

use std::f64::consts::PI;

const EPSILON: f64 = 0.00001;
const DEGREE_TO_RADIAN: f64 = (PI * 2.0) / 360.0;
const RADIAN_TO_DEGREE: f64 = 360.0 / (PI * 2.0);


pub fn approximately(a: f64, b: f64) -> bool {
    (a - b).abs() < EPSILON
}

pub fn degree_to_radian(degree: f64) -> f64 {
    degree * DEGREE_TO_RADIAN
}

pub fn radian_to_degree(radian: f64) -> f64 {
    radian * RADIAN_TO_DEGREE
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn approximately_returns_true_when_approximately_equal() {
        assert!(approximately(1.0, 1.0));
        assert!(approximately(1.0 + 0.000001, 1.0));
        assert!(approximately(1.0, 10.0 / 10.0));
    }

    #[test]
    fn approximately_returns_false_when_not_approximately_equal() {
        assert!(!approximately(1.0, 1.1));
        assert!(!approximately(1.0 + 0.001, 1.0));
    }

    #[test]
    fn test_degree_to_radian() {
        assert!(approximately(degree_to_radian(1.0), 0.0174533));
    }

    #[test]
    fn test_radian_to_degree() {
        assert!(approximately(radian_to_degree(1.0), 57.29577));
    }

}
