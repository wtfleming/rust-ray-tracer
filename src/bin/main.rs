use std::fs;

use rust_ray_tracer::canvas;
use rust_ray_tracer::color;
use rust_ray_tracer::mathf;
use rust_ray_tracer::mathf::matrix;
use rust_ray_tracer::mathf::vector3;
use rust_ray_tracer::ppm;

fn main() {
    // draw_simple();
    draw_clock();
}

#[allow(dead_code)]
fn draw_simple() {
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

fn draw_clock() {
    let mut canvas = canvas::new(100, 100);
    let radius = 30.0;
    let red = color::red();

    let origin = vector3::new(0.0, 0.0, 0.0);
    let to_center_of_canvas = vector3::new(50.0, 50.0, 0.0);

    // Draw a dot for every hour on a clock
    for x in 0..12 {
        let rotation_degrees = (x as f64) * (360.0 / 12.0);

        let rotation_mat = matrix::rotation_z(mathf::degree_to_radian(rotation_degrees));
        let translation_mat = matrix::translation(&vector3::new(radius, radius, 0.0));

        let position = rotation_mat
            .multiply_4x4(&translation_mat)
            .multiply_vector3(&origin)
            .add(&to_center_of_canvas);

        canvas.write_pixel(position.y as isize, position.x as isize, &red);
    }

    let ppm_data = ppm::canvas_to_ppm(&canvas);
    fs::write("renders/clock.ppm", ppm_data).expect("Unable to write file");
}
