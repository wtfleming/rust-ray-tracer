use crate::mathf::sphere;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct Intersection {
    pub t: f64,
    pub object: Rc<sphere::Sphere>,
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
        //mathf::approximately(self.x, other.x) && mathf::approximately(self.y, other.y) && mathf::approximately(self.z, other.z)
    }
}

#[cfg(test)]
mod tests {
    use super::super::approximately;
    use super::*;

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
}
