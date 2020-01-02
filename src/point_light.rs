use crate::color::Color;
use crate::mathf::vector3::Vector3;

#[derive(Debug)]
pub struct PointLight {
    pub position: Vector3,
    pub intensity: Color,
}

pub fn new(position: Vector3, intensity: Color) -> PointLight {
    PointLight {
        position: position,
        intensity,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color;
    use crate::mathf::vector3;

    #[test]
    fn a_point_light_has_a_position_and_intensity() {
        let intensity = color::new(1.0, 1.0, 1.0);
        let position = vector3::new(0.0, 0.0, 0.0);
        let light = new(position, intensity);

        assert_eq!(light.position, vector3::new(0.0, 0.0, 0.0));
        assert_eq!(light.intensity, color::new(1.0, 1.0, 1.0));
    }
}
