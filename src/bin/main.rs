use std::fs;

use rust_ray_tracer::mathf;

fn main() {
    //    println!("{:?}", mathf::vector3::new(1.0, 2.0, 3.0));

    let mut canvas = mathf::canvas::new(5, 3);
    let c1 = mathf::color::new(1.5, 0.0, 0.0);
    let c2 = mathf::color::new(0.0, 0.5, 0.0);
    let c3 = mathf::color::new(-0.5, 0.0, 1.0);
    canvas.write_pixel(0, 0, &c1);
    canvas.write_pixel(2, 1, &c2);
    canvas.write_pixel(4, 2, &c3);

    let ppm_data = mathf::ppm::canvas_to_ppm(&canvas);
    //    println!("{}", ppm_data);

    fs::write("renders/output.ppm", ppm_data).expect("Unable to write file");
}
