use crate::mathf::sphere;

//#[derive(Debug, Clone)]
pub struct Intersection {
    pub t: f64,
    pub object: sphere::Sphere,
}

pub struct Intersections {
    pub intersections: Vec<Intersection>,
}

pub fn new_intersections(i1: Intersection, i2: Intersection) -> Intersections {
    Intersections {
        intersections: vec![i1, i2],
    }
}

pub fn new(t: f64, object: sphere::Sphere) -> Intersection {
    Intersection { t, object }
}

#[cfg(test)]
mod tests {
    use super::super::approximately;
    use super::*;
    //    use crate::mathf::vector3;

    #[test]
    fn an_intersection_encapsulates_t_and_object() {
        let s = sphere::new();
        let s2 = s.clone();
        let i = new(3.5, s);
        assert!(approximately(i.t, 3.5));

        //assert_eq!(i.object, s);
        assert_eq!(i.object, s2);
    }

    // #[test]
    // fn aggregating_intersections() {
    //     let s = sphere::new();
    //     let i1 = new(1.0, s);
    //     let i2 = new(1.0, s);

    //     let xs = new_intersections(i1, i2);
    //     assert_eq!(xs.intersections.len(), 2);
    //     assert_eq!(xs.intersections[0].t, 1.0);
    //     assert_eq!(xs.intersections[1].t, 2.0);
    // }
}
