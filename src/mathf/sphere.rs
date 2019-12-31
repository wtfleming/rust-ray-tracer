use crate::mathf::matrix;
use crate::mathf::matrix::Matrix;


//#[derive(Debug, Clone)]
#[derive(Debug)]
pub struct Sphere {
    pub id: u32,
    pub transform: Matrix
}

pub fn new() -> Sphere {
    Sphere { id: sphere_id(), transform: matrix::identity_4x4()  }
}

impl PartialEq for Sphere {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

static mut SPHERE_ID: u32 = 0;

pub fn sphere_id() -> u32 {
    unsafe {
        SPHERE_ID += 1;
        SPHERE_ID
    }
}


impl Sphere {
    pub fn set_transform(&self, transform: Matrix) -> Sphere {
        Sphere { id: self.id, transform }
    }

}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::mathf::vector3;

    #[test]
    fn test_a_sphere_default_transformation() {
        let s = new();
        assert_eq!(s.transform, matrix::identity_4x4());
    }

    #[test]
    fn changing_a_sphere_transformation() {
        let s = new();
        let t = matrix::translation(&vector3::new(2.0, 3.0, 4.0));
        let s = s.set_transform(t);
        let expected = matrix::translation(&vector3::new(2.0, 3.0, 4.0));
        assert_eq!(s.transform, expected);
    }
}
