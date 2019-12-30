use crate::color;

pub struct Canvas {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<std::vec::Vec<color::Color>>,
}

pub fn new(width: usize, height: usize) -> Canvas {
    Canvas {
        width,
        height,
        pixels: vec![vec![color::new(0.0, 0.0, 0.0); height]; width],
    }
}


impl Canvas {
    pub fn write_pixel(&mut self, x: isize, y: isize, color: &color::Color) {
        // Note that x and y are positions in world space, they might not be in the camera's canvas

        // TODO is there a better way to handle the usize and isize in Rust?
        if x < 0 || y < 0 || (x as usize) >= self.width || (y as usize) >= self.height {
            // This is a part of the image not visible in the canvas, so do nothing
            return;
        }
        self.pixels[x as usize][y as usize] = color.clone();
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_creates_a_canvas() {
        let canvas = new(10, 20);
        assert_eq!(canvas.width, 10);
        assert_eq!(canvas.height, 20);
    }

    #[test]
    fn it_inits_all_colors_to_zero_when_creating_a_canvas() {
        let canvas = new(10, 20);
        let black = color::new(0.0, 0.0, 0.0);
        for color in canvas.pixels.iter().flat_map(|r| r.iter()) {
            assert!(color.equals(&black));
        }
    }

    #[test]
    fn test_canvas_write_pixel() {
        let mut canvas = new(10, 20);
        let red = color::new(1.0, 0.0, 0.0);
        canvas.write_pixel(2, 3, &red);
        assert!(canvas.pixels[2][3].equals(&red));

        let black = color::new(0.0, 0.0, 0.0);
        assert!(canvas.pixels[1][2].equals(&black));
    }
}
