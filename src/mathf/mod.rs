pub mod vector3;
pub mod vector4;
pub mod matrix;

const EPSILON: f64 = 0.00001;

pub fn approximately(a: f64, b: f64) -> bool {
    (a - b).abs() < EPSILON
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
}
