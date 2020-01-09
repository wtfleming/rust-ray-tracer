use crate::color::Color;
use crate::mathf::vector3::Vector3;

#[derive(Debug)]
pub struct PointLight {
    pub position: Vector3,
    pub intensity: Color,
}

impl PointLight {
    pub fn new(position: Vector3, intensity: Color) -> PointLight {
        PointLight {
            position,
            intensity,
        }
    }
}

impl PartialEq for PointLight {
    fn eq(&self, other: &Self) -> bool {
        self.position == other.position && self.intensity == other.intensity
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mathf::vector3::Vector3;

    #[test]
    fn a_point_light_has_a_position_and_intensity() {
        let intensity = Color::new(1.0, 1.0, 1.0);
        let position = Vector3::new(0.0, 0.0, 0.0);
        let light = PointLight::new(position, intensity);

        assert_eq!(light.position, Vector3::new(0.0, 0.0, 0.0));
        assert_eq!(light.intensity, Color::new(1.0, 1.0, 1.0));
    }
}
