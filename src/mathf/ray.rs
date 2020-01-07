use crate::mathf::matrix::Matrix;
use crate::mathf::vector3;
use crate::mathf::vector3::Vector3;
use crate::mathf::vector4;

#[derive(Debug)]
pub struct Ray {
    pub origin: Vector3,
    pub direction: Vector3,
}

impl Ray {
    pub fn new(origin: Vector3, direction: Vector3) -> Ray {
        Ray { origin, direction }
    }

    /// Compute the point at the given distance t along the ray
    pub fn position(&self, t: f64) -> Vector3 {
        &self.origin + &(&self.direction * t)
    }

    pub fn transform(&self, matrix: &Matrix) -> Ray {
        // We only want translation matrices to affect "points" and not "vectors".
        // By setting w to be 1 the point * transform = transformed point in space;
        // If w = 0 then point * transform = only rotated point.
        let origin = vector4::new(self.origin.x, self.origin.y, self.origin.z, 1.0);
        let direction = vector4::new(self.direction.x, self.direction.y, self.direction.z, 0.0);

        let origin = matrix.multiply_vector4(&origin);
        let direction = matrix.multiply_vector4(&direction);

        // Now convert back to a Vector3 representation
        let origin = vector3::new(origin.x, origin.y, origin.z);
        let direction = vector3::new(direction.x, direction.y, direction.z);
        Ray { origin, direction }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mathf::vector3;
    use crate::transformations;

    #[test]
    fn it_creates_a_ray() {
        let ray = Ray::new(vector3::new(1.0, 2.0, 3.0), vector3::new(4.0, 5.0, 6.0));
        assert_eq!(ray.origin, vector3::new(1.0, 2.0, 3.0));
        assert_eq!(ray.direction, vector3::new(4.0, 5.0, 6.0));
    }

    #[test]
    fn test_ray_position() {
        let ray = Ray::new(vector3::new(2.0, 3.0, 4.0), vector3::new(1.0, 0.0, 0.0));

        let position = ray.position(0.0);
        let expected = vector3::new(2.0, 3.0, 4.0);
        assert_eq!(position, expected);

        let position = ray.position(1.0);
        println!("{:?}", position);
        let expected = vector3::new(3.0, 3.0, 4.0);
        assert_eq!(position, expected);

        let position = ray.position(-1.0);
        let expected = vector3::new(1.0, 3.0, 4.0);
        assert_eq!(position, expected);

        let position = ray.position(2.5);
        let expected = vector3::new(4.5, 3.0, 4.0);
        assert_eq!(position, expected);
    }

    #[test]
    fn test_translating_a_ray() {
        let ray = Ray::new(vector3::new(1.0, 2.0, 3.0), vector3::new(0.0, 1.0, 0.0));
        let matrix = transformations::translation(&vector3::new(3.0, 4.0, 5.0));
        let ray2 = ray.transform(&matrix);
        assert_eq!(ray2.origin, vector3::new(4.0, 6.0, 8.0));
        assert_eq!(ray2.direction, vector3::new(0.0, 1.0, 0.0));
    }

    #[test]
    fn test_scaling_a_ray() {
        let ray = Ray::new(vector3::new(1.0, 2.0, 3.0), vector3::new(0.0, 1.0, 0.0));
        let matrix = transformations::scaling(&vector3::new(2.0, 3.0, 4.0));
        let ray2 = ray.transform(&matrix);
        assert_eq!(ray2.origin, vector3::new(2.0, 6.0, 12.0));
        assert_eq!(ray2.direction, vector3::new(0.0, 3.0, 0.0));
    }

}
