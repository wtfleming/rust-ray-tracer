use crate::mathf::intersection;
use crate::mathf::intersection::Intersection;
use crate::mathf::matrix::Matrix;
use crate::mathf::sphere;
use crate::mathf::vector3;
use crate::mathf::vector3::Vector3;
use crate::mathf::vector4;
use std::rc::Rc;

#[derive(Debug)]
pub struct Ray {
    pub origin: Vector3,
    pub direction: Vector3,
}

pub fn new(origin: Vector3, direction: Vector3) -> Ray {
    Ray { origin, direction }
}

pub fn hit(intersections: &intersection::Intersections) -> Option<Intersection> {
    // If there is a hit, it will be the intersection with the lowest nonnegative t value

    let mut result: Option<Intersection> = None;
    for i in &intersections.intersections {
        if i.t >= 0.0 {
            match &result {
                None => result = Some(i.clone()),
                Some(x) => {
                    if i.t < x.t {
                        result = Some(i.clone())
                    }
                }
            }
        }
    }

    result
}

impl Ray {
    /// Compute the point at the given distance t along the ray
    pub fn position(&self, t: f64) -> Vector3 {
        &self.origin + &(&self.direction * t)
    }

    pub fn intersect(&self, sphere: Rc<sphere::Sphere>) -> Vec<Intersection> {
        let ray2 = self.transform(&sphere.transform.inverse());

        let sphere_to_ray = &ray2.origin - &vector3::new(0.0, 0.0, 0.0);

        let a = ray2.direction.dot(&ray2.direction);
        let b = 2.0 * ray2.direction.dot(&sphere_to_ray);
        let c = sphere_to_ray.dot(&sphere_to_ray) - 1.0;
        let discriminant = (b * b) - (4.0 * a * c);

        if discriminant < 0.0 {
            // When the discrimint is negative then the ray missed and there were no intersections
            vec![]
        } else {
            let disc_root = discriminant.sqrt();
            let t1 = (-b - disc_root) / (2.0 * a);
            let t2 = (-b + disc_root) / (2.0 * a);

            let a = intersection::new(t1, Rc::clone(&sphere));
            let b = intersection::new(t2, Rc::clone(&sphere));
            vec![a, b]
        }
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
    use crate::mathf::matrix;
    use crate::mathf::vector3;

    #[test]
    fn it_creates_a_ray() {
        let ray = new(vector3::new(1.0, 2.0, 3.0), vector3::new(4.0, 5.0, 6.0));
        assert_eq!(ray.origin, vector3::new(1.0, 2.0, 3.0));
        assert_eq!(ray.direction, vector3::new(4.0, 5.0, 6.0));
    }

    #[test]
    fn test_ray_position() {
        let ray = new(vector3::new(2.0, 3.0, 4.0), vector3::new(1.0, 0.0, 0.0));

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
    fn a_ray_intersects_a_sphere_at_two_points() {
        let ray = new(vector3::new(0.0, 0.0, -5.0), vector3::new(0.0, 0.0, 1.0));
        let s = Rc::new(sphere::new());
        let xs = ray.intersect(s);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 4.0);
        assert_eq!(xs[1].t, 6.0);
    }

    #[test]
    fn a_ray_intersects_a_sphere_at_a_tangent() {
        let ray = new(vector3::new(0.0, 1.0, -5.0), vector3::new(0.0, 0.0, 1.0));
        let s = Rc::new(sphere::new());
        let xs = ray.intersect(s);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 5.0);
        assert_eq!(xs[1].t, 5.0);
    }

    #[test]
    fn a_ray_misses_a_sphere() {
        let ray = new(vector3::new(0.0, 2.0, -5.0), vector3::new(0.0, 0.0, 1.0));
        let s = Rc::new(sphere::new());
        let xs = ray.intersect(s);
        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn a_ray_originates_inside_a_sphere() {
        let ray = new(vector3::new(0.0, 0.0, 0.0), vector3::new(0.0, 0.0, 1.0));
        let s = Rc::new(sphere::new());
        let xs = ray.intersect(s);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, -1.0);
        assert_eq!(xs[1].t, 1.0);
    }

    #[test]
    fn a_sphere_is_behind_a_ray() {
        let ray = new(vector3::new(0.0, 0.0, 5.0), vector3::new(0.0, 0.0, 1.0));
        let s = Rc::new(sphere::new());
        let s2 = Rc::clone(&s);

        let xs = ray.intersect(s);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, -6.0);
        assert_eq!(xs[1].t, -4.0);

        assert_eq!(xs[0].object, s2);
        assert_eq!(xs[1].object, s2);
    }

    #[test]
    fn test_the_hit_when_all_intersections_have_positive_t() {
        let s = Rc::new(sphere::new());
        let i1 = intersection::new(1.0, Rc::clone(&s));
        let i1_copy = i1.clone();
        let i2 = intersection::new(2.0, Rc::clone(&s));
        let xs = intersection::new_intersections(vec![i2, i1]);
        let i = hit(&xs);

        assert_eq!(i.unwrap(), i1_copy);
    }

    #[test]
    fn test_the_hit_when_some_intersections_have_negative_t() {
        let s = Rc::new(sphere::new());
        let i1 = intersection::new(-1.0, Rc::clone(&s));
        let i2 = intersection::new(2.0, Rc::clone(&s));
        let i2_copy = i2.clone();
        let xs = intersection::new_intersections(vec![i2, i1]);
        let i = hit(&xs);

        assert_eq!(i.unwrap(), i2_copy);
    }

    #[test]
    fn test_the_hit_when_all_intersections_have_negative_t() {
        let s = Rc::new(sphere::new());
        let i1 = intersection::new(-2.0, Rc::clone(&s));
        let i2 = intersection::new(-1.0, Rc::clone(&s));
        let xs = intersection::new_intersections(vec![i2, i1]);
        let i = hit(&xs);

        assert!(i.is_none());
    }

    #[test]
    fn test_the_hit_is_always_the_lowest_nonnegative_intersection() {
        let s = Rc::new(sphere::new());
        let i1 = intersection::new(5.0, Rc::clone(&s));
        let i2 = intersection::new(7.0, Rc::clone(&s));
        let i3 = intersection::new(-3.0, Rc::clone(&s));
        let i4 = intersection::new(2.0, Rc::clone(&s));
        let i4_copy = i4.clone();
        let xs = intersection::new_intersections(vec![i1, i2, i3, i4]);
        let i = hit(&xs);

        assert_eq!(i.unwrap(), i4_copy);
    }

    #[test]
    fn test_translating_a_ray() {
        let ray = new(vector3::new(1.0, 2.0, 3.0), vector3::new(0.0, 1.0, 0.0));
        let matrix = matrix::translation(&vector3::new(3.0, 4.0, 5.0));
        let ray2 = ray.transform(&matrix);
        assert_eq!(ray2.origin, vector3::new(4.0, 6.0, 8.0));
        assert_eq!(ray2.direction, vector3::new(0.0, 1.0, 0.0));
    }

    #[test]
    fn test_scaling_a_ray() {
        let ray = new(vector3::new(1.0, 2.0, 3.0), vector3::new(0.0, 1.0, 0.0));
        let matrix = matrix::scaling(&vector3::new(2.0, 3.0, 4.0));
        let ray2 = ray.transform(&matrix);
        assert_eq!(ray2.origin, vector3::new(2.0, 6.0, 12.0));
        assert_eq!(ray2.direction, vector3::new(0.0, 3.0, 0.0));
    }

    #[test]
    fn intersecting_a_scaled_sphere_with_a_ray() {
        let ray = new(vector3::new(0.0, 0.0, -5.0), vector3::new(0.0, 0.0, 1.0));
        let s = sphere::new();
        let s = s.set_transform(matrix::scaling(&vector3::new(2.0, 2.0, 2.0)));
        let s = Rc::new(s);
        let xs = ray.intersect(s);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 3.0);
        assert_eq!(xs[1].t, 7.0);
    }

    #[test]
    fn intersecting_a_translated_sphere_with_a_ray() {
        let ray = new(vector3::new(0.0, 0.0, -5.0), vector3::new(0.0, 0.0, 1.0));
        let s = sphere::new();
        let s = s.set_transform(matrix::translation(&vector3::new(5.0, 0.0, 0.0)));
        let s = Rc::new(s);
        let xs = ray.intersect(s);
        assert_eq!(xs.len(), 0);
    }
}
