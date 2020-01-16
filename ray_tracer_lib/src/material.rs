use crate::color::Color;
use crate::mathf;

#[derive(Debug, Clone)]
pub struct Material {
    pub color: Color,
    pub ambient: f64,
    pub diffuse: f64,
    pub specular: f64,
    pub shininess: f64,
}

impl Default for Material {
    fn default() -> Self {
        Self::new()
    }
}

impl Material {
    pub fn new() -> Material {
        Material {
            color: Color::new(1.0, 1.0, 1.0),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
        }
    }
}

impl PartialEq for Material {
    fn eq(&self, other: &Self) -> bool {
        self.color == other.color
            && mathf::approximately(self.ambient, other.ambient)
            && mathf::approximately(self.diffuse, other.diffuse)
            && mathf::approximately(self.specular, other.specular)
            && mathf::approximately(self.shininess, other.shininess)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_the_default_material() {
        let material = Material::new();
        assert_eq!(material.color, Color::new(1.0, 1.0, 1.0));
        assert_eq!(material.ambient, 0.1);
        assert_eq!(material.diffuse, 0.9);
        assert_eq!(material.specular, 0.9);
        assert_eq!(material.shininess, 200.0);
    }
}
