use crate::mathf::ray::Ray;
use crate::mathf::sphere;
use crate::mathf::vector3::Vector3;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct Intersection {
    pub t: f64,
    pub object: Arc<sphere::Sphere>,
}

pub struct Computations {
    pub t: f64,
    pub object: Arc<sphere::Sphere>,
    pub point: Vector3,
    pub eye_vector: Vector3,
    pub normal_vector: Vector3,
    pub is_inside: bool,
}

pub struct Intersections {
    pub intersections: Vec<Intersection>,
}

impl PartialEq for Intersection {
    fn eq(&self, other: &Intersection) -> bool {
        self.t == other.t && &self.object == &other.object
    }
}

impl Intersections {
    pub fn new(intersections: Vec<Intersection>) -> Intersections {
        Intersections { intersections }
    }

    pub fn hit(&self) -> Option<Intersection> {
        // If there is a hit, it will be the intersection with the lowest nonnegative t value

        let mut result: Option<Intersection> = None;
        for i in &self.intersections {
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
}

impl Intersection {
    pub fn new(t: f64, object: Arc<sphere::Sphere>) -> Intersection {
        Intersection { t, object }
    }

    pub fn prepare_computations(&self, ray: Ray) -> Computations {
        let point = ray.position(self.t);
        let eye_vector = -ray.direction;
        let mut normal_vector = self.object.normal_at(&point);

        let is_inside;
        if normal_vector.dot(&eye_vector) < 0. {
            is_inside = true;
            normal_vector = -normal_vector;
        } else {
            is_inside = false;
        }

        Computations {
            t: self.t,
            object: Arc::clone(&self.object),
            point,
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
    use crate::mathf::vector3::Vector3;
    use crate::mathf::sphere::Sphere;

    #[test]
    fn an_intersection_encapsulates_t_and_object() {
        let s = Arc::new(Sphere::new(None, None));
        let i = Intersection::new(3.5, Arc::clone(&s));
        assert!(approximately(i.t, 3.5));
        assert_eq!(i.object, s);
    }

    #[test]
    fn aggregating_intersections() {
        let s = Arc::new(Sphere::new(None, None));
        let i1 = Intersection::new(1.0, Arc::clone(&s));
        let i2 = Intersection::new(2.0, Arc::clone(&s));

        let xs = Intersections::new(vec![i1, i2]);
        assert_eq!(xs.intersections.len(), 2);
        assert_eq!(xs.intersections[0].t, 1.0);
        assert_eq!(xs.intersections[1].t, 2.0);
    }

    #[test]
    fn precomputing_the_state_of_an_intersection() {
        let ray = Ray::new(Vector3::new(0., 0., -5.), Vector3::new(0., 0., 1.));
        let sphere = Arc::new(Sphere::new(None, None));
        let i = Intersection::new(4., Arc::clone(&sphere));

        let computations = i.prepare_computations(ray);
        assert_eq!(computations.t, i.t);
        assert_eq!(computations.object, sphere);
        assert_eq!(computations.point, Vector3::new(0., 0., -1.));
        assert_eq!(computations.eye_vector, Vector3::new(0., 0., -1.));
        assert_eq!(computations.normal_vector, Vector3::new(0., 0., -1.));
    }

    #[test]
    fn test_precomputing_the_hit_when_an_intersection_happens_on_the_outside() {
        let ray = Ray::new(Vector3::new(0., 0., -5.), Vector3::new(0., 0., 1.));
        let sphere = Arc::new(Sphere::new(None, None));
        let i = Intersection::new(4., Arc::clone(&sphere));

        let computations = i.prepare_computations(ray);
        assert!(!computations.is_inside);
    }

    #[test]
    fn test_precomputing_the_hit_when_an_intersection_happens_on_the_inside() {
        let ray = Ray::new(Vector3::new(0., 0., 0.), Vector3::new(0., 0., 1.));
        let sphere = Arc::new(Sphere::new(None, None));
        let i = Intersection::new(1., Arc::clone(&sphere));

        let computations = i.prepare_computations(ray);
        assert_eq!(computations.point, Vector3::new(0., 0., 1.));
        assert_eq!(computations.eye_vector, Vector3::new(0., 0., -1.));
        assert!(computations.is_inside);

        // Normal would have been (0, 0, 1), but is inverted
        assert_eq!(computations.normal_vector, Vector3::new(0., 0., -1.));
    }
}
