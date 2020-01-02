use crate::mathf;
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Debug, Clone)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

pub fn new(x: f64, y: f64, z: f64) -> Vector3 {
    Vector3 { x, y, z }
}


impl Vector3 {
    pub fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn normalize(&self) -> Vector3 {
        let mag = self.magnitude();
        new(self.x / mag, self.y / mag, self.z / mag)
    }

    pub fn dot(&self, rhs: &Vector3) -> f64 {
        (self.x * rhs.x) + (self.y * rhs.y) + (self.z * rhs.z)
    }

    pub fn cross(&self, rhs: &Vector3) -> Vector3 {
        new(
            self.y * rhs.z - self.z * rhs.y,
            self.z * rhs.x - self.x * rhs.z,
            self.x * rhs.y - self.y * rhs.x,
        )
    }
}



impl PartialEq for Vector3 {
    fn eq(&self, other: &Self) -> bool {
        mathf::approximately(self.x, other.x)
            && mathf::approximately(self.y, other.y)
            && mathf::approximately(self.z, other.z)
    }
}

// Should the 'a and 'b lifetimes be included like in the answer here?
// https://stackoverflow.com/a/28005283/148239
impl Add<&Vector3> for &Vector3 {
    type Output = Vector3;
    fn add(self, other: &Vector3) -> Vector3 {
        Vector3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub<&Vector3> for &Vector3 {
    type Output = Vector3;
    fn sub(self, other: &Vector3) -> Vector3 {
        Vector3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Mul<f64> for &Vector3 {
    type Output = Vector3;
    fn mul(self, other: f64) -> Vector3 {
        Vector3 {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl Mul<f64> for Vector3 {
    type Output = Vector3;
    fn mul(self, other: f64) -> Vector3 {
        Vector3 {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl Div<f64> for &Vector3 {
    type Output = Vector3;
    fn div(self, other: f64) -> Vector3 {
        Vector3 {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        }
    }
}

impl Neg for Vector3 {
    type Output = Vector3;
    fn neg(self) -> Vector3 {
        Vector3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}



#[cfg(test)]
mod tests {
    use super::super::approximately;
    use super::*;

    #[test]
    fn it_creates_a_vector3() {
        let vector = new(1.0, 2.0, 3.0);
        assert_eq!(vector.x, 1.0);
        assert_eq!(vector.y, 2.0);
        assert_eq!(vector.z, 3.0);
    }

    #[test]
    fn test_add_by_reference() {
        let a = new(3.0, -2.0, 5.0);
        let b = new(-2.0, 3.0, 1.0);
        let c = &a + &b;
        assert_eq!(c.x, 1.0);
        assert_eq!(c.y, 1.0);
        assert_eq!(c.z, 6.0);
    }

    #[test]
    fn test_subtract_by_reference() {
        let a = new(3.0, 2.0, 1.0);
        let b = new(5.0, 6.0, 7.0);
        let c = &a - &b;
        assert_eq!(c.x, -2.0);
        assert_eq!(c.y, -4.0);
        assert_eq!(c.z, -6.0);
    }

    #[test]
    fn it_multiplies_vector3s() {
        let a = new(1.0, -2.0, 3.0);
        let b = &a * 3.5;
        assert_eq!(b.x, 3.5);
        assert_eq!(b.y, -7.0);
        assert_eq!(b.z, 10.5);
    }

    #[test]
    fn test_divide() {
        let a = new(1.0, -2.0, 3.0);
        let b = &a / 2.0;
        assert_eq!(b.x, 0.5);
        assert_eq!(b.y, -1.0);
        assert_eq!(b.z, 1.5);
    }

    #[test]
    fn it_negates_vector3s() {
        let vector = new(1.0, -2.0, 3.0);
        let negated_vector = -vector;
        assert_eq!(negated_vector.x, -1.0);
        assert_eq!(negated_vector.y, 2.0);
        assert_eq!(negated_vector.z, -3.0);
    }

    #[test]
    fn test_vector3_magnitude() {
        let vector = new(0.0, 1.0, 0.0);
        assert_eq!(vector.magnitude(), 1.0);

        let vector = new(0.0, 0.0, 1.0);
        assert_eq!(vector.magnitude(), 1.0);

        let vector = new(1.0, 2.0, 3.0);
        assert_eq!(vector.magnitude(), 14.0f64.sqrt());

        let vector = new(-1.0, -2.0, -3.0);
        assert_eq!(vector.magnitude(), 14.0f64.sqrt());
    }

    #[test]
    fn test_vector3_normalize() {
        let vector = new(4.0, 0.0, 0.0);
        let normalized_vector = vector.normalize();
        assert_eq!(normalized_vector.x, 1.0);
        assert_eq!(normalized_vector.y, 0.0);
        assert_eq!(normalized_vector.z, 0.0);

        let vector = new(1.0, 2.0, 3.0);
        let normalized_vector = vector.normalize();
        assert!(approximately(normalized_vector.x, 0.26726));
        assert!(approximately(normalized_vector.y, 0.53452));
        assert!(approximately(normalized_vector.z, 0.80178));
    }

    #[test]
    fn test_normalized_vector3_has_magnitude_of_one() {
        let vector = new(1.0, 2.0, 3.0);
        let normalized_vector = vector.normalize();
        assert_eq!(normalized_vector.magnitude(), 1.0);
    }

    #[test]
    fn test_vector3_dot_product() {
        let a = new(1.0, 2.0, 3.0);
        let b = new(2.0, 3.0, 4.0);
        assert_eq!(a.dot(&b), 20.0);
    }

    #[test]
    fn test_vector3_cross_product() {
        let a = new(1.0, 2.0, 3.0);
        let b = new(2.0, 3.0, 4.0);

        let a_cross_b = a.cross(&b);
        assert_eq!(a_cross_b.x, -1.0);
        assert_eq!(a_cross_b.y, 2.0);
        assert_eq!(a_cross_b.z, -1.0);

        let b_cross_a = b.cross(&a);
        assert_eq!(b_cross_a.x, 1.0);
        assert_eq!(b_cross_a.y, -2.0);
        assert_eq!(b_cross_a.z, 1.0);
    }
}
