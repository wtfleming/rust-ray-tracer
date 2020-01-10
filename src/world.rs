use crate::color;
use crate::color::Color;
use crate::material::Material;
use crate::mathf::intersection::{Computations, Intersection, Intersections};
use crate::mathf::ray::Ray;
use crate::mathf::sphere::Sphere;
use crate::mathf::vector3::Vector3;
use crate::phong_lighting;
use crate::point_light::PointLight;
use crate::transformations;
use std::sync::Arc;

#[derive(Debug)]
pub struct World {
    pub light: Option<PointLight>,
    pub objects: Vec<Arc<Sphere>>,
}

pub fn new() -> World {
    World {
        light: None,
        objects: vec![],
    }
}

pub fn default_world() -> World {
    let light = PointLight::new(Vector3::new(-10., 10., -10.), Color::new(1., 1., 1.));

    let mut material = Material::new();
    material.color = Color::new(0.8, 1.0, 0.6);
    material.diffuse = 0.7;
    material.specular = 0.2;

    let s1 = Sphere::new(None, Some(material));
    let s1 = Arc::new(s1);

    let s2 = Sphere::new(Some(transformations::scaling(&Vector3::new(0.5, 0.5, 0.5))), None);
    let s2 = Arc::new(s2);

    World {
        light: Some(light),
        objects: vec![s1, s2],
    }
}

impl World {
    pub fn color_at(&self, ray: Ray) -> Color {
        let xs = self.intersect(&ray);
        match xs.hit() {
            None => color::BLACK,
            Some(i) => {
                let comps = i.prepare_computations(ray);
                self.shade_hit(comps)
            }
        }
    }


    fn intersect(&self, ray: &Ray) -> Intersections {
        let mut result: Vec<Intersection> = vec![];
        for object in self.objects.iter() {
            let i = Sphere::intersect(Arc::clone(&object), &ray);
            result.extend(i);
        }

        result.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());

        Intersections::new(result)
    }

    fn shade_hit(&self, computations: Computations) -> Color {
        // For now it's probably ok to just panic, but probably should handle this better?
        if self.light == None {
            panic!("You must add a light to a world before attempting to render it");
        }

        let shadowed = self.is_shadowed(&computations.over_point);

        // The world only supports one light at this time. To add additional ones we
        // would need to call the phong_lighting::lighting() function for each one,
        // and add the resulting colors together.

        phong_lighting::lighting(
            &computations.object.material(),
            &self.light.as_ref().unwrap(),
            &computations.point,
            &computations.eye_vector,
            &computations.normal_vector,
            shadowed,
        )
    }

    fn is_shadowed(&self, point: &Vector3) -> bool {
        let vector = &(self.light.as_ref().unwrap().position) - &point;
        let distance = vector.magnitude();
        let direction = vector.normalize();

        let ray = Ray::new(point.clone(), direction);
        let intersections = self.intersect(&ray);
        let hit = intersections.hit();

        hit.is_some() && hit.unwrap().t < distance
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creating_a_world() {
        let world = new();
        assert!(world.light.is_none());
        assert_eq!(world.objects.len(), 0);
    }

    #[test]
    fn test_creating_a_default_world() {
        let light = PointLight::new(Vector3::new(-10., 10., -10.), Color::new(1., 1., 1.));

        let world = default_world();
        assert_eq!(world.light.unwrap(), light);

        assert_eq!(world.objects.len(), 2);

        assert!(world
            .objects
            .iter()
            .any(|sphere| sphere.material().color == Color::new(0.8, 1.0, 0.6)));
        assert!(world
            .objects
            .iter()
            .any(|sphere| sphere.material().diffuse == 0.7));
        assert!(world
            .objects
            .iter()
            .any(|sphere| sphere.material().specular == 0.2));
        assert!(world
            .objects
            .iter()
            .any(|sphere| sphere.transform() == &transformations::scaling(&Vector3::new(0.5, 0.5, 0.5))));
    }

    #[test]
    fn test_intersect_a_world_with_a_ray() {
        let world = default_world();
        let ray = Ray::new(Vector3::new(0.0, 0.0, -5.0), Vector3::new(0.0, 0.0, 1.0));
        let xs = world.intersect(&ray);
        assert_eq!(xs.intersections.len(), 4);
        assert_eq!(xs.intersections[0].t, 4.0);
        assert_eq!(xs.intersections[1].t, 4.5);
        assert_eq!(xs.intersections[2].t, 5.5);
        assert_eq!(xs.intersections[3].t, 6.0);
    }

    #[test]
    fn test_shading_an_intersection() {
        let world = default_world();
        let ray = Ray::new(Vector3::new(0.0, 0.0, -5.0), Vector3::new(0.0, 0.0, 1.0));
        let shape = &world.objects[0];
        let intersection = Intersection::new(4., Arc::clone(&shape));
        let computations = intersection.prepare_computations(ray);
        let color = world.shade_hit(computations);

        assert_eq!(color, Color::new(0.38066, 0.47583, 0.2855));
    }

    #[test]
    fn test_shading_an_intersection_from_the_inside() {
        let mut world = default_world();
        world.light = Some(PointLight::new(
            Vector3::new(0., 0.25, 0.),
            Color::new(1., 1., 1.),
        ));

        let ray = Ray::new(Vector3::new(0.0, 0.0, 0.0), Vector3::new(0.0, 0.0, 1.0));
        let shape = &world.objects[1];
        let intersection = Intersection::new(0.5, Arc::clone(&shape));
        let computations = intersection.prepare_computations(ray);
        let color = world.shade_hit(computations);

        assert_eq!(color, Color::new(0.90498, 0.90498, 0.90498));
    }

    #[test]
    fn test_the_color_when_a_ray_misses() {
        let world = default_world();
        let ray = Ray::new(Vector3::new(0.0, 0.0, -5.0), Vector3::new(0.0, 1.0, 0.0));
        let color = world.color_at(ray);
        assert_eq!(color, Color::new(0., 0., 0.)); // Black
    }

    #[test]
    fn test_the_color_when_a_ray_hits() {
        let world = default_world();
        let ray = Ray::new(Vector3::new(0.0, 0.0, -5.0), Vector3::new(0.0, 0.0, 1.0));
        let color = world.color_at(ray);
        assert_eq!(color, Color::new(0.38066, 0.47583, 0.2855));
    }

    #[test]
    fn test_the_color_with_an_intersection_behind_the_ray() {
        let world = {
            let light = PointLight::new(Vector3::new(-10., 10., -10.), Color::new(1., 1., 1.));

            let mut material = Material::new();
            material.color = Color::new(0.8, 1.0, 0.6);
            material.ambient = 1.0;
            material.diffuse = 0.7;
            material.specular = 0.2;

            let s1 = Sphere::new(None, Some(material));
            let s1 = Arc::new(s1);

            let mut material = Material::new();
            material.ambient = 1.0;
            let s2 = Sphere::new(Some(transformations::scaling(&Vector3::new(0.5, 0.5, 0.5))), Some(material));
            let s2 = Arc::new(s2);

            World {
                light: Some(light),
                objects: vec![s1, s2],
            }
        };

        let inner_color = world.objects[1].material().color.clone();

        let ray = Ray::new(Vector3::new(0.0, 0.0, 0.75), Vector3::new(0.0, 0.0, -1.0));
        let color = world.color_at(ray);
        assert_eq!(color, inner_color);
    }

    #[test]
    fn there_is_no_shadow_when_nothing_is_colinear_with_point_and_light() {
        let world = default_world();
        let point = Vector3::new(0., 10., 0.);
        assert!(!world.is_shadowed(&point));
    }

    #[test]
    fn the_shadow_when_an_object_is_between_the_point_and_the_light() {
        let world = default_world();
        let point = Vector3::new(10., -10., 10.);
        assert!(world.is_shadowed(&point));
    }

    #[test]
    fn there_is_no_shadow_when_an_object_is_behind_the_light() {
        let world = default_world();
        let point = Vector3::new(-20., 20., -20.);
        assert!(!world.is_shadowed(&point));
    }

    #[test]
    fn there_is_no_shadow_when_an_object_is_behind_the_point() {
        let world = default_world();
        let point = Vector3::new(-2., 2., -2.);
        assert!(!world.is_shadowed(&point));
    }

    #[test]
    fn shade_hit_is_given_an_intersection_in_shadow() {
        let s1 = Sphere::new(None, None);
        let s1 = Arc::new(s1);

        let s2 = Sphere::new(Some(transformations::translation(&Vector3::new(0., 0., 10.))), None);
        let s2 = Arc::new(s2);
        let s2_clone = s2.clone();

        let world = {
            let light = PointLight::new(Vector3::new(0., 0., -10.), Color::new(1., 1., 1.));

            World {
                light: Some(light),
                objects: vec![s1, s2],
            }
        };

        let ray = Ray::new(Vector3::new(0., 0., 5.), Vector3::new(0., 0., 1.));
        let intersection = Intersection::new(4., Arc::clone(&s2_clone));
        let comps = intersection.prepare_computations(ray);
        let color = world.shade_hit(comps);
        assert_eq!(color, Color::new(0.1, 0.1, 0.1));
    }
}
