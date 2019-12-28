use std::fs;

use rust_ray_tracer::canvas;
use rust_ray_tracer::color;
use rust_ray_tracer::ppm;

fn main() {
    //    println!("{:?}", mathf::vector3::new(1.0, 2.0, 3.0));

    let mut canvas = canvas::new(5, 3);
    let c1 = color::new(1.5, 0.0, 0.0);
    let c2 = color::new(0.0, 0.5, 0.0);
    let c3 = color::new(-0.5, 0.0, 1.0);
    canvas.write_pixel(0, 0, &c1);
    canvas.write_pixel(2, 1, &c2);
    canvas.write_pixel(4, 2, &c3);

    let ppm_data = ppm::canvas_to_ppm(&canvas);
    //    println!("{}", ppm_data);

    fs::write("renders/output.ppm", ppm_data).expect("Unable to write file");
}
