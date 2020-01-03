use crate::material;
use crate::material::Material;
use crate::mathf::matrix;
use crate::mathf::matrix::Matrix;
use crate::mathf::vector3;
use crate::mathf::vector3::Vector3;

#[derive(Debug)]
pub struct Sphere {
    pub id: u32,
    pub material: Material,
    pub transform: Matrix,
}

pub fn new() -> Sphere {
    Sphere {
        id: sphere_id(),
        transform: matrix::identity_4x4(),
        material: material::new(),
    }
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
    pub fn normal_at(&self, world_point: &Vector3) -> Vector3 {
        let object_point = self.transform.inverse().multiply_vector3(&world_point);
        let object_normal = &object_point - &vector3::new(0.0, 0.0, 0.0);
        let world_normal = self
            .transform
            .inverse()
            .transpose()
            .multiply_vector3(&object_normal);
        world_normal.normalize()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::material;
    use crate::mathf::vector3;
    use std::f64::consts::PI;

    #[test]
    fn test_a_sphere_default_transformation() {
        let s = new();
        assert_eq!(s.transform, matrix::identity_4x4());
    }

    #[test]
    fn changing_a_sphere_transformation() {
        let mut s = new();
        let t = matrix::translation(&vector3::new(2.0, 3.0, 4.0));
        s.transform = t;
        let expected = matrix::translation(&vector3::new(2.0, 3.0, 4.0));
        assert_eq!(s.transform, expected);
    }

    #[test]
    fn the_normal_on_a_sphere_at_a_point_on_the_x_axis() {
        let s = new();
        let n = s.normal_at(&vector3::new(1.0, 0.0, 0.0));
        assert_eq!(n, vector3::new(1.0, 0.0, 0.0));
    }

    #[test]
    fn the_normal_on_a_sphere_at_a_point_on_the_y_axis() {
        let s = new();
        let n = s.normal_at(&vector3::new(0.0, 1.0, 0.0));
        assert_eq!(n, vector3::new(0.0, 1.0, 0.0));
    }

    #[test]
    fn the_normal_on_a_sphere_at_a_point_on_the_z_axis() {
        let s = new();
        let n = s.normal_at(&vector3::new(0.0, 0.0, 1.0));
        assert_eq!(n, vector3::new(0.0, 0.0, 1.0));
    }

    #[test]
    fn the_normal_on_a_sphere_at_a_nonaxial_point() {
        let s = new();
        let n = s.normal_at(&vector3::new(
            3.0f64.sqrt() / 3.0,
            3.0f64.sqrt() / 3.0,
            3.0f64.sqrt() / 3.0,
        ));
        assert_eq!(
            n,
            vector3::new(
                3.0f64.sqrt() / 3.0,
                3.0f64.sqrt() / 3.0,
                3.0f64.sqrt() / 3.0
            )
        );
    }

    #[test]
    fn the_normal_is_a_normalized_vector() {
        let s = new();
        let n = s.normal_at(&vector3::new(
            3.0f64.sqrt() / 3.0,
            3.0f64.sqrt() / 3.0,
            3.0f64.sqrt() / 3.0,
        ));
        assert_eq!(n, n.normalize());
    }

    #[test]
    fn computing_the_normal_on_a_translated_sphere() {
        let mut s = new();
        let t = matrix::translation(&vector3::new(0.0, 1.0, 0.0));
        s.transform = t;
        let n = s.normal_at(&vector3::new(0.0, 1.70711, -0.70711));
        assert_eq!(n, vector3::new(0.0, 0.70711, -0.70711));
    }

    #[test]
    fn computing_the_normal_on_a_transformed_sphere() {
        let mut s = new();
        let m = matrix::scaling(&vector3::new(1.0, 0.5, 1.0))
            .multiply_4x4(&matrix::rotation_z(PI / 5.0));
        s.transform = m;
        let n = s.normal_at(&vector3::new(
            0.0,
            2.0f64.sqrt() / 2.0,
            -(2.0f64.sqrt() / 2.0),
        ));
        assert_eq!(n, vector3::new(0.0, 0.97014, -0.24254));
    }

    #[test]
    fn reflecting_a_vector_approaching_at_45_degrees() {
        let v = vector3::new(1.0, -1.0, 0.0);
        let n = vector3::new(0.0, 1.0, 0.0);
        let r = reflect(&v, &n);
        assert_eq!(r, vector3::new(1.0, 1.0, 0.0));
    }

    #[test]
    fn reflecting_a_vector_off_a_slanted_surface() {
        let v = vector3::new(0.0, -1.0, 0.0);
        let n = vector3::new(2.0f64.sqrt() / 2.0, 2.0f64.sqrt() / 2.0, 0.0);
        let r = reflect(&v, &n);
        assert_eq!(r, vector3::new(1.0, 0.0, 0.0));
    }

    #[test]
    fn a_sphere_has_a_default_material() {
        let s = new();
        let m = material::new();
        assert_eq!(s.material, m);
    }

    #[test]
    fn a_sphere_may_be_assigned_a_material() {
        let mut sphere = new();
        let mut m = material::new();
        m.ambient = 1.0;
        sphere.material = m;

        let mut m2 = material::new();
        m2.ambient = 1.0;

        assert_eq!(sphere.material, m2);
    }
}
