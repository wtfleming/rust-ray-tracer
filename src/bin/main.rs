use std::fs;

use rust_ray_tracer::canvas;
use rust_ray_tracer::color;
use rust_ray_tracer::ppm;

fn main() {
    let mut canvas = canvas::new(5, 3);
    let red = color::red();
    let green = color::green();
    let blue = color::blue();
    canvas.write_pixel(0, 0, &red);
    canvas.write_pixel(2, 1, &green);
    canvas.write_pixel(4, 2, &blue);

    let ppm_data = ppm::canvas_to_ppm(&canvas);
    fs::write("renders/output.ppm", ppm_data).expect("Unable to write file");
}
