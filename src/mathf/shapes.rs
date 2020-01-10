use crate::material::Material;
use crate::mathf::intersection::Intersection;
use crate::mathf::matrix::Matrix;
use crate::mathf::ray::Ray;
use crate::mathf::vector3::Vector3;
use std::fmt;
use std::sync::Arc;

pub trait Shape: fmt::Debug + Send + Sync {
    fn transform(&self) -> &Matrix;
    fn inverse_transform(&self) -> &Matrix;
    fn material(&self) -> &Material;
    fn local_intersect(&self, shape: Arc<dyn Shape>, object_ray: Ray) -> Vec<Intersection>;
    fn local_normal_at(&self, object_point: Vector3) -> Vector3;
    fn local_eq(&self, other: &dyn Shape) -> bool;

    fn intersect(&self, shape: Arc<dyn Shape>, world_ray: Ray) -> Vec<Intersection> {
        self.local_intersect(shape, world_ray.transform(self.inverse_transform()))
    }

    fn normal_at(&self, world_point: Vector3) -> Vector3 {
        let object_normal =
            self.local_normal_at(self.inverse_transform().multiply_point(&world_point));
        let world_normal = self
            .inverse_transform()
            .transpose()
            .multiply_vector(&object_normal);
        world_normal.normalize()
    }
}

impl PartialEq for dyn Shape {
    fn eq(&self, other: &dyn Shape) -> bool {
        self.local_eq(other)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mathf::vector3::Vector3;
    use crate::transformations;
    use std::f64::consts::PI;

    static mut SAVED_RAY: Ray = Ray {
        origin: crate::mathf::vector3::ORIGIN,
        direction: crate::mathf::vector3::VECTOR_Y_UP,
    };

    #[derive(Debug)]
    pub struct TestShape {
        material: Material,
        transform: Matrix,
        inverse_transform: Matrix,
    }

    impl Shape for TestShape {
        fn transform(&self) -> &Matrix {
            &self.transform
        }
        fn inverse_transform(&self) -> &Matrix {
            &self.inverse_transform
        }
        fn material(&self) -> &Material {
            &self.material
        }

        fn local_intersect(&self, _shape: Arc<dyn Shape>, object_ray: Ray) -> Vec<Intersection> {
            unsafe {
                SAVED_RAY = object_ray;
            }
            vec![]
        }

        fn local_normal_at(&self, object_point: Vector3) -> Vector3 {
            Vector3::new(object_point.x, object_point.y, object_point.z)
        }

        fn local_eq(&self, other: &dyn Shape) -> bool {
            self.material() == other.material() || self.transform() == other.transform()
        }
    }

    impl TestShape {
        pub fn new(transform: Option<Matrix>, material: Option<Material>) -> TestShape {
            let t = match transform {
                None => Matrix::identity_4x4(),
                Some(x) => x,
            };
            let inverse_transform = t.inverse().unwrap();
            let mat = material.unwrap_or_default();

            TestShape {
                transform: t,
                material: mat,
                inverse_transform,
            }
        }
    }

    #[test]
    fn the_default_transformation() {
        let s = TestShape::new(None, None);
        assert_eq!(s.transform(), &Matrix::identity_4x4());
    }

    #[test]
    fn assigning_a_transformation() {
        let t = transformations::translation(&Vector3::new(2.0, 3.0, 4.0));
        let s = TestShape::new(Some(t.clone()), None);
        assert_eq!(s.transform(), &t);
    }

    #[test]
    fn test_inverse_transform() {
        let t = transformations::translation(&Vector3::new(2.0, 3.0, 4.0));
        let s = TestShape::new(Some(t.clone()), None);

        let inverse_t = transformations::translation(&Vector3::new(-2.0, -3.0, -4.0));
        assert_eq!(s.inverse_transform(), &inverse_t);
    }

    #[test]
    fn the_default_material() {
        let s = TestShape::new(None, None);
        assert_eq!(s.material(), &Material::new());
    }

    #[test]
    fn assigning_a_material() {
        let mut mat = Material::new();
        mat.ambient = 1.;
        let s = TestShape::new(None, Some(mat));
        assert_eq!(s.material().ambient, 1.);
    }

    #[test]
    fn test_intersect_scaled_shape_with_ray() {
        let ray = Ray::new(Vector3::new(0., 0., -5.), Vector3::new(0., 0., 1.));
        let t = transformations::scaling(&Vector3::new(2.0, 2.0, 2.0));
        let shape = Arc::new(TestShape::new(Some(t), None));
        let shape2 = Arc::clone(&shape);

        let _xs = shape.intersect(shape2, ray);
        unsafe {
            assert_eq!(SAVED_RAY.origin, Vector3::new(0., 0., -2.5));
            assert_eq!(SAVED_RAY.direction, Vector3::new(0., 0., 0.5));
        }
    }

    #[test]
    fn test_intersect_translated_shape_with_ray() {
        let ray = Ray::new(Vector3::new(0., 0., -5.), Vector3::new(0., 0., 1.));
        let t = transformations::translation(&Vector3::new(5.0, 0.0, 0.0));
        let shape = Arc::new(TestShape::new(Some(t), None));
        let shape2 = Arc::clone(&shape);

        let _xs = shape.intersect(shape2, ray);
        unsafe {
            assert_eq!(SAVED_RAY.origin, Vector3::new(-5., 0., -5.));
            assert_eq!(SAVED_RAY.direction, Vector3::new(0., 0., 1.));
        }
    }

    #[test]
    fn computing_the_normal_on_a_translated_shape() {
        let t = transformations::translation(&Vector3::new(0., 1., 0.));
        let shape = TestShape::new(Some(t), None);
        let normal = shape.normal_at(Vector3::new(0., 1.70711, -0.70711));
        assert_eq!(normal, Vector3::new(0., 0.70711, -0.70711));
    }

    #[test]
    fn computing_the_normal_on_a_transformed_shape() {
        let t = transformations::scaling(&Vector3::new(1., 0.5, 1.))
            .multiply_4x4(&transformations::rotation_z(PI / 5.));
        let shape = TestShape::new(Some(t), None);
        let normal = shape.normal_at(Vector3::new(0., 2f64.sqrt() / 2., -2f64.sqrt() / 2.));
        assert_eq!(normal, Vector3::new(0., 0.97014, -0.24254));
    }
}
