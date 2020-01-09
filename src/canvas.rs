use crate::color::Color;

pub struct Canvas {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<std::vec::Vec<Color>>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Canvas {
        Canvas {
            width,
            height,
            pixels: vec![vec![Color::new(0.0, 0.0, 0.0); width]; height],
        }
    }

    pub fn write_pixel(&mut self, x: isize, y: isize, color: &Color) {
        // Note that x and y are positions in world space, they might not be in the camera's canvas

        // if x < 0 || y < 0 || (x as usize) >= self.width || (y as usize) >= self.height {
        //     // This is a part of the image not visible in the canvas, so do nothing
        //     return;
        // }
        self.pixels[y as usize][x as usize] = color.clone();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_creates_a_canvas() {
        let canvas = Canvas::new(10, 20);
        assert_eq!(canvas.width, 10);
        assert_eq!(canvas.height, 20);
    }

    #[test]
    fn it_inits_all_colors_to_zero_when_creating_a_canvas() {
        let canvas = Canvas::new(10, 20);
        for color in canvas.pixels.iter().flat_map(|r| r.iter()) {
            let black = Color::new(0.0, 0.0, 0.0);
            assert!(color == &black);
        }
    }

    #[test]
    fn test_canvas_write_pixel() {
        let mut canvas = Canvas::new(10, 20);
        let red = Color::new(1.0, 0.0, 0.0);
        canvas.write_pixel(2, 3, &red);
        assert!(canvas.pixels[3][2] == red);

        let black = Color::new(0.0, 0.0, 0.0);
        assert!(canvas.pixels[2][1] == black);
    }
}
