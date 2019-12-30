use crate::mathf::vector3;
use crate::mathf::vector3::Vector3;

#[derive(Debug)]
pub struct Ray {
    pub origin: Vector3,
    pub direction: Vector3,
}

pub fn new(origin: Vector3, direction: Vector3) -> Ray {
    Ray { origin, direction }
}

static mut SPHERE_ID: u32 = 0;

pub fn sphere() -> u32 {
    unsafe {
        SPHERE_ID += 1;
        SPHERE_ID
    }
}

impl Ray {
    /// Compute the point at the given distance t along the ray
    pub fn position(&self, t: f64) -> Vector3 {
        self.origin.add(&self.direction.multiply(t))
    }

    pub fn intersect(&self, _sphere_id: u32) -> Vec<f64> {
        let sphere_to_ray = self.origin.subtract(&vector3::new(0.0, 0.0, 0.0));

        let a = self.direction.dot(&self.direction);
        let b = 2.0 * self.direction.dot(&sphere_to_ray);
        let c = sphere_to_ray.dot(&sphere_to_ray) - 1.0;
        let discriminant = (b * b) - (4.0 * a * c);

        if discriminant < 0.0 {
            // When the discrimint is negative then the ray missed and there were no intersections
            vec![]
        } else {
            let disc_root = discriminant.sqrt();
            let t1 = (-b - disc_root) / (2.0 * a);
            let t2 = (-b + disc_root) / (2.0 * a);
            vec![t1, t2]
        }
    }
}

#[cfg(test)]
mod tests {
    //    use super::super::approximately;
    use super::*;
    use crate::mathf::vector3;

    #[test]
    fn it_creates_a_ray() {
        let ray = new(vector3::new(1.0, 2.0, 3.0), vector3::new(4.0, 5.0, 6.0));
        assert!(ray.origin.equals(&vector3::new(1.0, 2.0, 3.0)));
        assert!(ray.direction.equals(&vector3::new(4.0, 5.0, 6.0)));
    }

    #[test]
    fn test_ray_position() {
        let ray = new(vector3::new(2.0, 3.0, 4.0), vector3::new(1.0, 0.0, 0.0));

        let position = ray.position(0.0);
        let expected = vector3::new(2.0, 3.0, 4.0);
        assert!(position.equals(&expected));

        let position = ray.position(1.0);
        println!("{:?}", position);
        let expected = vector3::new(3.0, 3.0, 4.0);
        assert!(position.equals(&expected));

        let position = ray.position(-1.0);
        let expected = vector3::new(1.0, 3.0, 4.0);
        assert!(position.equals(&expected));

        let position = ray.position(2.5);
        let expected = vector3::new(4.5, 3.0, 4.0);
        assert!(position.equals(&expected));
    }

    #[test]
    fn a_ray_intersects_a_sphere_at_two_points() {
        let ray = new(vector3::new(0.0, 0.0, -5.0), vector3::new(0.0, 0.0, 1.0));
        let s = sphere();
        let xs = ray.intersect(s);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0], 4.0);
        assert_eq!(xs[1], 6.0);
    }

    #[test]
    fn a_ray_intersects_a_sphere_at_a_tangent() {
        let ray = new(vector3::new(0.0, 1.0, -5.0), vector3::new(0.0, 0.0, 1.0));
        let s = sphere();
        let xs = ray.intersect(s);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0], 5.0);
        assert_eq!(xs[1], 5.0);
    }

    #[test]
    fn a_ray_misses_a_sphere() {
        let ray = new(vector3::new(0.0, 2.0, -5.0), vector3::new(0.0, 0.0, 1.0));
        let s = sphere();
        let xs = ray.intersect(s);
        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn a_ray_originates_inside_a_sphere() {
        let ray = new(vector3::new(0.0, 0.0, 0.0), vector3::new(0.0, 0.0, 1.0));
        let s = sphere();
        let xs = ray.intersect(s);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0], -1.0);
        assert_eq!(xs[1], 1.0);
    }

    #[test]
    fn a_sphere_is_behind_a_ray() {
        let ray = new(vector3::new(0.0, 0.0, 5.0), vector3::new(0.0, 0.0, 1.0));
        let s = sphere();
        let xs = ray.intersect(s);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0], -6.0);
        assert_eq!(xs[1], -4.0);
    }
}
