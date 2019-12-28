// Code to convert a Canvas to PPM format
// http://netpbm.sourceforge.net/doc/ppm.html

use crate::canvas;
use crate::color;

pub fn canvas_to_ppm(canvas: &canvas::Canvas) -> String {
    ppm_header(&canvas) + &ppm_pixel_data(canvas)
}

fn f64_to_ppm_pixel(value: f64) -> u8 {
    let clamped = num::clamp(value, 0.0, 1.0);
    (clamped * 255.0).ceil() as u8
}

fn color_to_ppm_pixel(color: &color::Color) -> String {
    let red = f64_to_ppm_pixel(color.r);
    let green = f64_to_ppm_pixel(color.g);
    let blue = f64_to_ppm_pixel(color.b);
    format!("{} {} {}", red, green, blue)
}

fn ppm_header(canvas: &canvas::Canvas) -> String {
    // Identifier of the flavor of PPM we are using
    let line1 = String::from("P3");
    let line2 = format!("{} {}", canvas.width, canvas.height);

    // Max color value. Here we indicate we will scale to be from 0 to 255
    let line3 = String::from("255");
    format!("{}\n{}\n{}\n", line1, line2, line3)
}

fn ppm_pixel_data(canvas: &canvas::Canvas) -> String {
    let mut pixel_data = String::from("");
    for color in canvas.pixels.iter().flat_map(|r| r.iter()) {
        let out = format!("{}\n", color_to_ppm_pixel(&color));
        pixel_data.push_str(&out);
    }
    pixel_data
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::canvas::new;

    #[test]
    fn it_creates_a_correct_ppm_header() {
        let canvas = new(5, 3);
        let ppm = canvas_to_ppm(&canvas);
        let split = ppm.split("\n").collect::<Vec<_>>();
        assert_eq!(split[0], "P3");
        assert_eq!(split[1], "5 3");
        assert_eq!(split[2], "255");
    }

    #[test]
    fn it_creates_the_correct_ppm_pixel_data() {
        let mut canvas = new(5, 3);
        let c1 = color::new(1.5, 0.0, 0.0);
        let c2 = color::new(0.0, 0.5, 0.0);
        let c3 = color::new(-0.5, 0.0, 1.0);
        canvas.write_pixel(0, 0, &c1);
        canvas.write_pixel(2, 1, &c2);
        canvas.write_pixel(4, 2, &c3);

        let ppm = canvas_to_ppm(&canvas);
        let split = ppm.split("\n").collect::<Vec<_>>();
        assert_eq!(split[3], "255 0 0");
    }

    #[test]
    fn test_color_to_ppm_pixel() {
        let c1 = color::new(1.5, 0.0, 0.0);
        assert_eq!(color_to_ppm_pixel(&c1), "255 0 0");

        let c2 = color::new(0.0, 0.5, 0.0);
        assert_eq!(color_to_ppm_pixel(&c2), "0 128 0");

        let c3 = color::new(-0.5, 0.0, 1.0);
        assert_eq!(color_to_ppm_pixel(&c3), "0 0 255");
    }

    #[test]
    fn ppm_files_are_terminated_with_newline() {
        let canvas = new(1, 1);
        let ppm = canvas_to_ppm(&canvas);
        let split = ppm.split("\n").collect::<Vec<_>>();
        assert_eq!(split[0], "P3");
        assert_eq!(split[1], "1 1");
        assert_eq!(split[2], "255");
        assert_eq!(split[3], "0 0 0");
        assert_eq!(split[4], "");
    }
}
