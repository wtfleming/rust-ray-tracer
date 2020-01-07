use crate::material::Material;
use crate::mathf::intersection::Intersection;
use crate::mathf::matrix::Matrix;
use crate::mathf::ray::Ray;
use crate::mathf::vector3::Vector3;

use std::rc::Rc;

#[derive(Debug)]
pub struct Sphere {
    id: u32,
    material: Material,
    transform: Matrix,
    inverse_transform: Matrix,
}

pub fn reflect(vector: &Vector3, normal: &Vector3) -> Vector3 {
    vector - &(normal * 2.0 * vector.dot(&normal))
}

impl PartialEq for Sphere {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

// TODO there is likely a better way to handle this than using an unsafe block
static mut SPHERE_ID: u32 = 0;

pub fn sphere_id() -> u32 {
    unsafe {
        SPHERE_ID += 1;
        SPHERE_ID
    }
}

impl Sphere {
    pub fn new(transform: Option<Matrix>, material: Option<Material>) -> Sphere {
        let t = match transform {
            None => Matrix::identity_4x4(),
            Some(x) => x,
        };
        let mat = match material {
            None => Material::new(),
            Some(x) => x,
        };
        let inverse_transform = t.inverse().clone();

        Sphere {
            id: sphere_id(),
            transform: t,
            material: mat,
            inverse_transform,
        }
    }

    pub fn material(&self) -> &Material {
        &self.material
    }

    pub fn transform(&self) -> &Matrix {
        &self.transform
    }

    pub fn inverse_transform(&self) -> &Matrix {
        &self.inverse_transform
    }

    pub fn normal_at(&self, world_point: &Vector3) -> Vector3 {
        let object_point = self.transform.inverse().multiply_vector3(&world_point);
        let object_normal = &object_point - &Vector3::new(0.0, 0.0, 0.0);
        let world_normal = self
            .transform
            .inverse()
            .transpose()
            .multiply_vector3(&object_normal);
        world_normal.normalize()
    }

    pub fn intersect(sphere: Rc<Sphere>, world_ray: &Ray) -> Vec<Intersection> {
        let object_ray = world_ray.transform(&sphere.inverse_transform());

        let sphere_to_ray = &object_ray.origin - &Vector3::new(0.0, 0.0, 0.0);

        // println!("{:?}", sphere_to_ray); // TODO THIS SEEMS TO ALWAYS BE THE SAME FOR EACH PIXEL - IF SO CAN CACHE IT ON THE SPHERE OBJECT?

        let a = object_ray.direction.dot(&object_ray.direction);
        let b = 2. * object_ray.direction.dot(&sphere_to_ray);
        let c = sphere_to_ray.dot(&sphere_to_ray) - 1.;
        let discriminant = (b * b) - (4. * a * c);

        if discriminant < 0.0 {
            // When the discrimint is negative then the ray missed and there were no intersections
            vec![]
        } else {
            let disc_root = discriminant.sqrt();
            let t1 = (-b - disc_root) / (2. * a);
            let t2 = (-b + disc_root) / (2. * a);

            let a = Intersection::new(t1, Rc::clone(&sphere));
            let b = Intersection::new(t2, Rc::clone(&sphere));
            vec![a, b]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mathf::intersection::Intersections;
    use crate::mathf::vector3::Vector3;
    use crate::transformations;
    use std::f64::consts::PI;

    #[test]
    fn test_a_sphere_default_transformation() {
        let s = Sphere::new(None, None);
        assert_eq!(s.transform, Matrix::identity_4x4());
    }

    #[test]
    fn changing_a_sphere_transformation() {
        let mut s = Sphere::new(None, None);
        let t = transformations::translation(&Vector3::new(2.0, 3.0, 4.0));
        s.transform = t;
        let expected = transformations::translation(&Vector3::new(2.0, 3.0, 4.0));
        assert_eq!(s.transform, expected);
    }

    #[test]
    fn the_normal_on_a_sphere_at_a_point_on_the_x_axis() {
        let s = Sphere::new(None, None);
        let n = s.normal_at(&Vector3::new(1.0, 0.0, 0.0));
        assert_eq!(n, Vector3::new(1.0, 0.0, 0.0));
    }

    #[test]
    fn the_normal_on_a_sphere_at_a_point_on_the_y_axis() {
        let s = Sphere::new(None, None);
        let n = s.normal_at(&Vector3::new(0.0, 1.0, 0.0));
        assert_eq!(n, Vector3::new(0.0, 1.0, 0.0));
    }

    #[test]
    fn the_normal_on_a_sphere_at_a_point_on_the_z_axis() {
        let s = Sphere::new(None, None);
        let n = s.normal_at(&Vector3::new(0.0, 0.0, 1.0));
        assert_eq!(n, Vector3::new(0.0, 0.0, 1.0));
    }

    #[test]
    fn the_normal_on_a_sphere_at_a_nonaxial_point() {
        let s = Sphere::new(None, None);
        let n = s.normal_at(&Vector3::new(
            3.0f64.sqrt() / 3.0,
            3.0f64.sqrt() / 3.0,
            3.0f64.sqrt() / 3.0,
        ));
        assert_eq!(
            n,
            Vector3::new(
                3.0f64.sqrt() / 3.0,
                3.0f64.sqrt() / 3.0,
                3.0f64.sqrt() / 3.0
            )
        );
    }

    #[test]
    fn the_normal_is_a_normalized_vector() {
        let s = Sphere::new(None, None);
        let n = s.normal_at(&Vector3::new(
            3.0f64.sqrt() / 3.0,
            3.0f64.sqrt() / 3.0,
            3.0f64.sqrt() / 3.0,
        ));
        assert_eq!(n, n.normalize());
    }

    #[test]
    fn computing_the_normal_on_a_translated_sphere() {
        let s = Sphere::new(
            Some(transformations::translation(&Vector3::new(0.0, 1.0, 0.0))),
            None,
        );
        let n = s.normal_at(&Vector3::new(0.0, 1.70711, -0.70711));
        assert_eq!(n, Vector3::new(0.0, 0.70711, -0.70711));
    }

    #[test]
    fn computing_the_normal_on_a_transformed_sphere() {
        let m = transformations::scaling(&Vector3::new(1.0, 0.5, 1.0))
            .multiply_4x4(&transformations::rotation_z(PI / 5.0));
        let s = Sphere::new(Some(m), None);
        let n = s.normal_at(&Vector3::new(
            0.0,
            2.0f64.sqrt() / 2.0,
            -(2.0f64.sqrt() / 2.0),
        ));
        assert_eq!(n, Vector3::new(0.0, 0.97014, -0.24254));
    }

    #[test]
    fn reflecting_a_vector_approaching_at_45_degrees() {
        let v = Vector3::new(1.0, -1.0, 0.0);
        let n = Vector3::new(0.0, 1.0, 0.0);
        let r = reflect(&v, &n);
        assert_eq!(r, Vector3::new(1.0, 1.0, 0.0));
    }

    #[test]
    fn reflecting_a_vector_off_a_slanted_surface() {
        let v = Vector3::new(0.0, -1.0, 0.0);
        let n = Vector3::new(2.0f64.sqrt() / 2.0, 2.0f64.sqrt() / 2.0, 0.0);
        let r = reflect(&v, &n);
        assert_eq!(r, Vector3::new(1.0, 0.0, 0.0));
    }

    #[test]
    fn a_sphere_has_a_default_material() {
        let s = Sphere::new(None, None);
        let m = Material::new();
        assert_eq!(s.material, m);
    }

    #[test]
    fn a_sphere_may_be_assigned_a_material() {
        let mut m = Material::new();
        m.ambient = 1.0;
        let sphere = Sphere::new(None, Some(m));

        let mut m2 = Material::new();
        m2.ambient = 1.0;

        assert_eq!(sphere.material, m2);
    }

    #[test]
    fn intersecting_a_scaled_sphere_with_a_ray() {
        let ray = Ray::new(Vector3::new(0.0, 0.0, -5.0), Vector3::new(0.0, 0.0, 1.0));
        let s = Sphere::new(
            Some(transformations::scaling(&Vector3::new(2.0, 2.0, 2.0))),
            None,
        );
        let s = Rc::new(s);
        let xs = Sphere::intersect(s, &ray);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 3.0);
        assert_eq!(xs[1].t, 7.0);
    }

    #[test]
    fn intersecting_a_translated_sphere_with_a_ray() {
        let ray = Ray::new(Vector3::new(0.0, 0.0, -5.0), Vector3::new(0.0, 0.0, 1.0));
        let s = Sphere::new(
            Some(transformations::translation(&Vector3::new(5.0, 0.0, 0.0))),
            None,
        );
        let s = Rc::new(s);
        let xs = Sphere::intersect(s, &ray);
        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn a_ray_intersects_a_sphere_at_two_points() {
        let ray = Ray::new(Vector3::new(0.0, 0.0, -5.0), Vector3::new(0.0, 0.0, 1.0));
        let s = Rc::new(Sphere::new(None, None));
        let xs = Sphere::intersect(s, &ray);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 4.0);
        assert_eq!(xs[1].t, 6.0);
    }

    #[test]
    fn a_ray_intersects_a_sphere_at_a_tangent() {
        let ray = Ray::new(Vector3::new(0.0, 1.0, -5.0), Vector3::new(0.0, 0.0, 1.0));
        let s = Rc::new(Sphere::new(None, None));
        let xs = Sphere::intersect(s, &ray);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 5.0);
        assert_eq!(xs[1].t, 5.0);
    }

    #[test]
    fn a_ray_misses_a_sphere() {
        let ray = Ray::new(Vector3::new(0.0, 2.0, -5.0), Vector3::new(0.0, 0.0, 1.0));
        let s = Rc::new(Sphere::new(None, None));
        let xs = Sphere::intersect(s, &ray);
        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn a_ray_originates_inside_a_sphere() {
        let ray = Ray::new(Vector3::new(0.0, 0.0, 0.0), Vector3::new(0.0, 0.0, 1.0));
        let s = Rc::new(Sphere::new(None, None));
        let xs = Sphere::intersect(s, &ray);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, -1.0);
        assert_eq!(xs[1].t, 1.0);
    }

    #[test]
    fn a_sphere_is_behind_a_ray() {
        let ray = Ray::new(Vector3::new(0.0, 0.0, 5.0), Vector3::new(0.0, 0.0, 1.0));
        let s = Rc::new(Sphere::new(None, None));
        let s2 = Rc::clone(&s);

        let xs = Sphere::intersect(s, &ray);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, -6.0);
        assert_eq!(xs[1].t, -4.0);

        assert_eq!(xs[0].object, s2);
        assert_eq!(xs[1].object, s2);
    }

    #[test]
    fn test_the_hit_when_all_intersections_have_positive_t() {
        let s = Rc::new(Sphere::new(None, None));
        let i1 = Intersection::new(1.0, Rc::clone(&s));
        let i1_copy = i1.clone();
        let i2 = Intersection::new(2.0, Rc::clone(&s));
        let xs = Intersections::new(vec![i2, i1]);
        let i = xs.hit();

        assert_eq!(i.unwrap(), i1_copy);
    }

    #[test]
    fn test_the_hit_when_some_intersections_have_negative_t() {
        let s = Rc::new(Sphere::new(None, None));
        let i1 = Intersection::new(-1.0, Rc::clone(&s));
        let i2 = Intersection::new(2.0, Rc::clone(&s));
        let i2_copy = i2.clone();
        let xs = Intersections::new(vec![i2, i1]);
        let i = xs.hit();

        assert_eq!(i.unwrap(), i2_copy);
    }

    #[test]
    fn test_the_hit_when_all_intersections_have_negative_t() {
        let s = Rc::new(Sphere::new(None, None));
        let i1 = Intersection::new(-2.0, Rc::clone(&s));
        let i2 = Intersection::new(-1.0, Rc::clone(&s));
        let xs = Intersections::new(vec![i2, i1]);
        let i = xs.hit();

        assert!(i.is_none());
    }

    #[test]
    fn test_the_hit_is_always_the_lowest_nonnegative_intersection() {
        let s = Rc::new(Sphere::new(None, None));
        let i1 = Intersection::new(5.0, Rc::clone(&s));
        let i2 = Intersection::new(7.0, Rc::clone(&s));
        let i3 = Intersection::new(-3.0, Rc::clone(&s));
        let i4 = Intersection::new(2.0, Rc::clone(&s));
        let i4_copy = i4.clone();
        let xs = Intersections::new(vec![i1, i2, i3, i4]);
        let i = xs.hit();

        assert_eq!(i.unwrap(), i4_copy);
    }
}
