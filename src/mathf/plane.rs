use crate::material::Material;
use crate::mathf;
use crate::mathf::intersection::Intersection;
use crate::mathf::matrix::Matrix;
use crate::mathf::ray::Ray;
use crate::mathf::shapes::Shape;
use crate::mathf::vector3::Vector3;
use std::sync::Arc;

#[derive(Debug)]
pub struct Plane {
    //    id: u32,
    material: Material,
    transform: Matrix,
    inverse_transform: Matrix,
}

impl Shape for Plane {
    fn material(&self) -> &Material {
        &self.material
    }

    fn transform(&self) -> &Matrix {
        &self.transform
    }

    fn inverse_transform(&self) -> &Matrix {
        &self.inverse_transform
    }

    fn local_intersect(&self, shape: Arc<dyn Shape>, object_ray: Ray) -> Vec<Intersection> {
        if object_ray.direction.y.abs() < mathf::EPSILON {
            return vec![];
        }

        // Note that this formula only works if this is a plane stretching
        // infinitely far in the x and z dimensions (which is the case here).
        let t = -object_ray.origin.y / object_ray.direction.y;

        let i = Intersection::new(t, Arc::clone(&shape));
        vec![i]
    }

    fn local_normal_at(&self, _object_point: Vector3) -> Vector3 {
        Vector3::new(0., 1., 0.)
    }

    fn local_eq(&self, other: &dyn Shape) -> bool {
        //        self.id == other.id
        self.material() == other.material() || self.transform() == other.transform()
    }
}

impl Plane {
    pub fn new(transform: Option<Matrix>, material: Option<Material>) -> Plane {
        let t = match transform {
            None => Matrix::identity_4x4(),
            Some(x) => x,
        };
        let inverse_transform = t.inverse().unwrap();
        let mat = material.unwrap_or_default();
        Plane {
            //            id: sphere_id(),
            transform: t,
            material: mat,
            inverse_transform,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mathf::vector3::Vector3;

    #[test]
    fn the_normal_of_a_plane_is_constant_everywhere() {
        let plane = Plane::new(None, None);
        let n1 = plane.local_normal_at(Vector3::new(0., 0., 0.));
        let n2 = plane.local_normal_at(Vector3::new(10., 0., -10.));
        let n3 = plane.local_normal_at(Vector3::new(-5., 0., 150.));

        assert_eq!(n1, Vector3::new(0., 1., 0.));
        assert_eq!(n2, Vector3::new(0., 1., 0.));
        assert_eq!(n3, Vector3::new(0., 1., 0.));
    }

    #[test]
    fn test_intersect_with_a_ray_parallel_to_the_plane() {
        let plane = Plane::new(None, None);
        let plane: Arc<dyn Shape> = Arc::new(plane);

        let ray = Ray::new(Vector3::new(0., 10., 0.), Vector3::new(0., 0., 1.));
        let xs = plane.local_intersect(Arc::clone(&plane), ray);
        assert!(xs.is_empty());
    }

    #[test]
    fn test_intersect_with_a_coplanar_ray() {
        let plane = Plane::new(None, None);
        let plane: Arc<dyn Shape> = Arc::new(plane);

        let ray = Ray::new(Vector3::new(0., 0., 0.), Vector3::new(0., 0., 1.));
        let xs = plane.local_intersect(Arc::clone(&plane), ray);
        assert!(xs.is_empty());
    }

    #[test]
    fn test_a_ray_intersecting_a_plane_from_above() {
        let plane = Plane::new(None, None);
        let plane: Arc<dyn Shape> = Arc::new(plane);

        let ray = Ray::new(Vector3::new(0., 1., 0.), Vector3::new(0., -1., 0.));
        let xs = plane.local_intersect(Arc::clone(&plane), ray);
        println!("{:?}", xs);
        assert_eq!(xs.len(), 1);
        assert_eq!(xs[0].t, 1.);
        assert_eq!(&xs[0].object, &Arc::clone(&plane));
    }

    #[test]
    fn test_a_ray_intersecting_a_plane_from_below() {
        let plane = Plane::new(None, None);
        let plane: Arc<dyn Shape> = Arc::new(plane);

        let ray = Ray::new(Vector3::new(0., -1., 0.), Vector3::new(0., 1., 0.));
        let xs = plane.local_intersect(Arc::clone(&plane), ray);
        assert_eq!(xs.len(), 1);
        assert_eq!(xs[0].t, 1.);
        assert_eq!(&xs[0].object, &Arc::clone(&plane));
    }
}
