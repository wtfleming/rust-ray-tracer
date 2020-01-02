use crate::color;
use crate::color::Color;
use crate::material::Material;
use crate::mathf::sphere;
use crate::mathf::vector3::Vector3;
use crate::point_light::PointLight;

pub fn lighting(
    material: &Material,
    light: &PointLight,
    point: &Vector3,
    eye_vector: &Vector3,
    normal_vector: &Vector3,
) -> Color {
    let diffuse;
    let specular;

    // Combine the surface color with the light's color/intensity
    let effective_color = material.color.multiply_color(&light.intensity);

    // Find the direction to the light source
    let light_vector = light.position.subtract(point).normalize();

    // Compute the ambient contribution
    let ambient = effective_color.multiply_scalar(material.ambient);

    // light_dot_normal represents the cosine of the angle between the light
    // vector and the normal vector. A negative number means the light is
    // on the other side of the surface.
    let light_dot_normal = light_vector.dot(&normal_vector);

    if light_dot_normal < 0.0 {
        diffuse = color::black();
        specular = color::black();
    } else {
        // Compute the diffuse contribution
        diffuse = effective_color
            .multiply_scalar(material.diffuse)
            .multiply_scalar(light_dot_normal);

        // reflect_dot_eye represents the cosine of the angle between the reflection
        // vector and the eye vector. A negative number means the light reflects
        // away from the eye.
        let reflect_vector = sphere::reflect(&light_vector.negate(), normal_vector);
        let reflect_dot_eye = reflect_vector.dot(&eye_vector);

        if reflect_dot_eye <= 0.0 {
            specular = color::black();
        } else {
            // Compute the specular contribution
            let factor = reflect_dot_eye.powf(material.shininess);
            specular = light
                .intensity
                .multiply_scalar(material.specular)
                .multiply_scalar(factor);
        }
    }

    ambient.add(&diffuse).add(&specular)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color;
    use crate::material;
    use crate::mathf::vector3;
    use crate::point_light;

    #[test]
    fn lighting_with_the_eye_between_the_light_and_surface() {
        let material = material::new();
        let position = vector3::new(0.0, 0.0, 0.0);

        let eye_vector = vector3::new(0.0, 0.0, -1.0);
        let normal_vector = vector3::new(0.0, 0.0, -1.0);
        let light = point_light::new(vector3::new(0.0, 0.0, -10.0), color::new(1.0, 1.0, 1.0));
        let result = lighting(&material, &light, &position, &eye_vector, &normal_vector);

        assert_eq!(result, color::new(1.9, 1.9, 1.9));
    }

    #[test]
    fn lighting_with_the_eye_between_the_light_and_surface_eye_offset_45_degrees() {
        let material = material::new();
        let position = vector3::new(0.0, 0.0, 0.0);

        let eye_vector = vector3::new(0.0, 2.0f64.sqrt() / 2.0, -2.0f64.sqrt() / 2.0);
        let normal_vector = vector3::new(0.0, 0.0, -1.0);
        let light = point_light::new(vector3::new(0.0, 0.0, -10.0), color::new(1.0, 1.0, 1.0));
        let result = lighting(&material, &light, &position, &eye_vector, &normal_vector);

        assert_eq!(result, color::new(1.0, 1.0, 1.0));
    }

    #[test]
    fn lighting_with_eye_opposite_surface() {
        let material = material::new();
        let position = vector3::new(0.0, 0.0, 0.0);

        let eye_vector = vector3::new(0.0, 0.0, -1.0);
        let normal_vector = vector3::new(0.0, 0.0, -1.0);
        let light = point_light::new(vector3::new(0.0, 10.0, -10.0), color::new(1.0, 1.0, 1.0));
        let result = lighting(&material, &light, &position, &eye_vector, &normal_vector);

        assert_eq!(result, color::new(0.73640, 0.73640, 0.73640));
    }

    #[test]
    fn lighting_with_the_eye_in_the_path_of_the_reflection_vector() {
        let material = material::new();
        let position = vector3::new(0.0, 0.0, 0.0);

        let eye_vector = vector3::new(0.0, -2.0f64.sqrt() / 2.0, -2.0f64.sqrt() / 2.0);
        let normal_vector = vector3::new(0.0, 0.0, -1.0);
        let light = point_light::new(vector3::new(0.0, 10.0, -10.0), color::new(1.0, 1.0, 1.0));
        let result = lighting(&material, &light, &position, &eye_vector, &normal_vector);

        assert_eq!(result, color::new(1.6364, 1.6364, 1.6364));
    }

    #[test]
    fn lighting_with_the_light_behind_the_surface() {
        let material = material::new();
        let position = vector3::new(0.0, 0.0, 0.0);

        let eye_vector = vector3::new(0.0, 0.0, -1.0);
        let normal_vector = vector3::new(0.0, 0.0, -1.0);
        let light = point_light::new(vector3::new(0.0, 0.0, 10.0), color::new(1.0, 1.0, 1.0));
        let result = lighting(&material, &light, &position, &eye_vector, &normal_vector);

        assert_eq!(result, color::new(0.1, 0.1, 0.1));
    }
}
