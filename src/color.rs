use crate::mathf::approximately;

#[derive(Debug, Clone)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

pub fn new(r: f64, g: f64, b: f64) -> Color {
    Color { r, g, b }
}

impl Color {
    pub fn add(&self, rhs: &Color) -> Color {
        new(self.r + rhs.r, self.g + rhs.g, self.b + rhs.b)
    }

    pub fn subtract(&self, rhs: &Color) -> Color {
        new(self.r - rhs.r, self.g - rhs.g, self.b - rhs.b)
    }

    pub fn multiply_scalar(&self, rhs: f64) -> Color {
        new(self.r * rhs, self.g * rhs, self.b * rhs)
    }

    // Hadamard Product
    pub fn multiply_color(&self, rhs: &Color) -> Color {
        new(self.r * rhs.r, self.g * rhs.g, self.b * rhs.b)
    }

    pub fn equals(&self, rhs: &Color) -> bool {
        approximately(self.r, rhs.r) && approximately(self.g, rhs.g) && approximately(self.b, rhs.b)
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_creates_a_color() {
        let color = new(0.5, 0.4, 1.7);
        assert!(approximately(color.r, 0.5));
        assert!(approximately(color.g, 0.4));
        assert!(approximately(color.b, 1.7));
    }

    #[test]
    fn it_adds_colors() {
        let a = new(0.9, 0.6, 0.75);
        let b = new(0.7, 0.1, 0.25);
        let c = a.add(&b);
        assert!(approximately(c.r, 1.6));
        assert!(approximately(c.g, 0.7));
        assert!(approximately(c.b, 1.0));
    }

    #[test]
    fn it_subtracts_colors() {
        let a = new(0.9, 0.6, 0.75);
        let b = new(0.7, 0.1, 0.25);
        let c = a.subtract(&b);
        assert!(approximately(c.r, 0.2));
        assert!(approximately(c.g, 0.5));
        assert!(approximately(c.b, 0.5));
    }

    #[test]
    fn it_multiplies_color_by_a_scalar() {
        let a = new(0.2, 0.3, 0.4);
        let b = a.multiply_scalar(2.0);
        assert!(approximately(b.r, 0.4));
        assert!(approximately(b.g, 0.6));
        assert!(approximately(b.b, 0.8));
    }

    #[test]
    fn it_multiplies_color_by_a_color() {
        let a = new(1.0, 0.2, 0.4);
        let b = new(0.9, 1.0, 0.1);
        let c = a.multiply_color(&b);
        assert!(approximately(c.r, 0.9));
        assert!(approximately(c.g, 0.2));
        assert!(approximately(c.b, 0.04));
    }

    #[test]
    fn test_color_equals() {
        let color = new(0.5, 0.4, 1.7);
        let same_color = new(0.5, 0.4, 1.7);
        let different_color = new(0.1, 0.2, 1.3);
        assert!(color.equals(&same_color));
        assert!(!color.equals(&different_color));
    }

}
