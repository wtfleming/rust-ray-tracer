use crate::canvas;
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

/// Convert a color value in the range 0.0 to 1.0 into 0 to 255
fn convert_rbg_value_to_byte(value: f64) -> u8 {
    // Values can be above 1.0, if so clamp them to the correct range for output
    let clamped = num::clamp(value, 0.0, 1.0);
    (clamped * 255.0).ceil() as u8
}

pub fn save_canvas_to_png(canvas: &canvas::Canvas, file_path: String) {
    let path = Path::new(&file_path);
    let file = File::create(path).unwrap();
    let w = BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, canvas.width as u32, canvas.height as u32);
    encoder.set_color(png::ColorType::RGB);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();

    const BYTES_PER_PIXEL: usize = 3;
    let mut image_data: Vec<u8> = vec![0u8; canvas.width * canvas.height * BYTES_PER_PIXEL];
    let mut i: usize = 0;
    for color in canvas.pixels.iter().flat_map(|r| r.iter()) {
        image_data[i] = convert_rbg_value_to_byte(color.r);
        image_data[i + 1] = convert_rbg_value_to_byte(color.g);
        image_data[i + 2] = convert_rbg_value_to_byte(color.b);
        i += 3;
    }

    writer.write_image_data(&image_data).unwrap(); // Save
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_rbg_value_to_byte() {
        assert_eq!(convert_rbg_value_to_byte(1.0), 255);
        assert_eq!(convert_rbg_value_to_byte(1.5), 255);
        assert_eq!(convert_rbg_value_to_byte(0.), 0);
        assert_eq!(convert_rbg_value_to_byte(0.5), 128);
    }
}
