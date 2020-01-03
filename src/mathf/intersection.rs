use crate::mathf::ray::Ray;
use crate::mathf::sphere;
use crate::mathf::vector3::Vector3;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct Intersection {
    pub t: f64,
    pub object: Rc<sphere::Sphere>,
}

pub struct Computations {
    pub t: f64,
    pub object: Rc<sphere::Sphere>,
    pub point: Vector3,
    pub eye_vector: Vector3,
    pub normal_vector: Vector3,
    pub is_inside: bool
}


pub struct Intersections {
    pub intersections: Vec<Intersection>,
}

pub fn new_intersections(intersections: Vec<Intersection>) -> Intersections {
    Intersections { intersections }
}

pub fn new(t: f64, object: Rc<sphere::Sphere>) -> Intersection {
    Intersection { t, object }
}

impl PartialEq for Intersection {
    fn eq(&self, other: &Self) -> bool {
        self.t == other.t && self.object == other.object
    }
}

impl Intersection {
    pub fn prepare_computations(&self, ray: &Ray) -> Computations {
        let ray_position = ray.position(self.t);

        let eye_vector = -(ray.direction.clone());
        let mut normal_vector = self.object.normal_at(&ray_position);

        let is_inside;
        if normal_vector.dot(&eye_vector) < 0. {
            is_inside = true;
            normal_vector = -normal_vector;
        } else {
            is_inside = false;
        }

        Computations {
            t: self.t,
            object: Rc::clone(&self.object),
            point: ray_position,
            eye_vector,
            normal_vector,
            is_inside,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mathf::approximately;
    use crate::mathf::ray;
    use crate::mathf::vector3;


    #[test]
    fn an_intersection_encapsulates_t_and_object() {
        let s = Rc::new(sphere::new());
        let i = new(3.5, Rc::clone(&s));
        assert!(approximately(i.t, 3.5));
        assert_eq!(i.object, s);
    }

    #[test]
    fn aggregating_intersections() {
        let s = Rc::new(sphere::new());
        let i1 = new(1.0, Rc::clone(&s));
        let i2 = new(2.0, Rc::clone(&s));

        let xs = new_intersections(vec![i1, i2]);
        assert_eq!(xs.intersections.len(), 2);
        assert_eq!(xs.intersections[0].t, 1.0);
        assert_eq!(xs.intersections[1].t, 2.0);
    }

    #[test]
    fn precomputing_the_state_of_an_intersection() {
        let ray = ray::new(vector3::new(0., 0., -5.), vector3::new(0., 0., 1.));
        let sphere = Rc::new(sphere::new());
        let i = new(4., Rc::clone(&sphere));

        let computations = i.prepare_computations(&ray);
        assert_eq!(computations.t, i.t);
        assert_eq!(computations.object, sphere);
        assert_eq!(computations.point, vector3::new(0., 0., -1.));
        assert_eq!(computations.eye_vector, vector3::new(0., 0., -1.));
        assert_eq!(computations.normal_vector, vector3::new(0., 0., -1.));
    }

    #[test]
    fn test_precomputing_the_hit_when_an_intersection_happens_on_the_outside() {
        let ray = ray::new(vector3::new(0., 0., -5.), vector3::new(0., 0., 1.));
        let sphere = Rc::new(sphere::new());
        let i = new(4., Rc::clone(&sphere));

        let computations = i.prepare_computations(&ray);
        assert!(!computations.is_inside);
    }

    #[test]
    fn test_precomputing_the_hit_when_an_intersection_happens_on_the_inside() {
        let ray = ray::new(vector3::new(0., 0., 0.), vector3::new(0., 0., 1.));
        let sphere = Rc::new(sphere::new());
        let i = new(1., Rc::clone(&sphere));

        let computations = i.prepare_computations(&ray);
        assert_eq!(computations.point, vector3::new(0., 0., 1.));
        assert_eq!(computations.eye_vector, vector3::new(0., 0., -1.));
        assert!(computations.is_inside);

        // Normal would have been (0, 0, 1), but is inverted
        assert_eq!(computations.normal_vector, vector3::new(0., 0., -1.));
    }

}
