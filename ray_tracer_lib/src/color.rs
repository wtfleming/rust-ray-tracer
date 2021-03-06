use crate::mathf;
use std::ops;

#[derive(Debug, Clone)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Color {
        Color { r, g, b }
    }
}

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        mathf::approximately(self.r, other.r)
            && mathf::approximately(self.g, other.g)
            && mathf::approximately(self.b, other.b)
    }
}

pub const RED: Color = Color {
    r: 1.,
    g: 0.,
    b: 0.,
};
pub const GREEN: Color = Color {
    r: 0.,
    g: 1.,
    b: 0.,
};
pub const BLUE: Color = Color {
    r: 0.,
    g: 0.,
    b: 1.,
};
pub const BLACK: Color = Color {
    r: 0.,
    g: 0.,
    b: 0.,
};
pub const WHITE: Color = Color {
    r: 1.,
    g: 1.,
    b: 1.,
};

impl ops::Add for Color {
    type Output = Color;
    fn add(self, other: Color) -> Color {
        Color {
            r: self.r + other.r,
            g: self.g + other.g,
            b: self.b + other.b,
        }
    }
}

impl ops::Sub for Color {
    type Output = Color;
    fn sub(self, other: Color) -> Color {
        Color {
            r: self.r - other.r,
            g: self.g - other.g,
            b: self.b - other.b,
        }
    }
}

impl ops::Mul<f64> for Color {
    type Output = Color;
    fn mul(self, other: f64) -> Color {
        Color {
            r: self.r * other,
            g: self.g * other,
            b: self.b * other,
        }
    }
}

impl ops::Mul<f64> for &Color {
    type Output = Color;
    fn mul(self, other: f64) -> Color {
        Color {
            r: self.r * other,
            g: self.g * other,
            b: self.b * other,
        }
    }
}

impl ops::Mul<Color> for Color {
    type Output = Color;
    fn mul(self, other: Color) -> Color {
        // Hadamard Product
        Color {
            r: self.r * other.r,
            g: self.g * other.g,
            b: self.b * other.b,
        }
    }
}

impl ops::Mul<&Color> for &Color {
    type Output = Color;
    fn mul(self, other: &Color) -> Color {
        // Hadamard Product
        Color {
            r: self.r * other.r,
            g: self.g * other.g,
            b: self.b * other.b,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_creates_a_color() {
        let color = Color::new(0.5, 0.4, 1.7);
        assert!(mathf::approximately(color.r, 0.5));
        assert!(mathf::approximately(color.g, 0.4));
        assert!(mathf::approximately(color.b, 1.7));
    }

    #[test]
    fn it_adds_colors() {
        let a = Color::new(0.9, 0.6, 0.75);
        let b = Color::new(0.7, 0.1, 0.25);
        let c = a + b;
        assert!(mathf::approximately(c.r, 1.6));
        assert!(mathf::approximately(c.g, 0.7));
        assert!(mathf::approximately(c.b, 1.0));
    }

    #[test]
    fn it_subtracts_colors() {
        let a = Color::new(0.9, 0.6, 0.75);
        let b = Color::new(0.7, 0.1, 0.25);
        let c = a - b;
        assert!(mathf::approximately(c.r, 0.2));
        assert!(mathf::approximately(c.g, 0.5));
        assert!(mathf::approximately(c.b, 0.5));
    }

    #[test]
    fn it_multiplies_color_by_a_scalar() {
        let a = Color::new(0.2, 0.3, 0.4);
        let b = a * 2.0;
        assert!(mathf::approximately(b.r, 0.4));
        assert!(mathf::approximately(b.g, 0.6));
        assert!(mathf::approximately(b.b, 0.8));
    }

    #[test]
    fn it_multiplies_color_by_a_color() {
        let a = Color::new(1.0, 0.2, 0.4);
        let b = Color::new(0.9, 1.0, 0.1);
        let c = a * b;
        assert!(mathf::approximately(c.r, 0.9));
        assert!(mathf::approximately(c.g, 0.2));
        assert!(mathf::approximately(c.b, 0.04));
    }

    #[test]
    fn test_color_equals() {
        let color = Color::new(0.5, 0.4, 1.7);
        let same_color = Color::new(0.5, 0.4, 1.7);
        let different_color = Color::new(0.1, 0.2, 1.3);
        assert!(color == same_color);
        assert!(color != different_color);
    }
}
