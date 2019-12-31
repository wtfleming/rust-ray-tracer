use std::fs;
use std::rc::Rc;

use rust_ray_tracer::canvas;
use rust_ray_tracer::color;
use rust_ray_tracer::mathf;
use rust_ray_tracer::mathf::matrix;
use rust_ray_tracer::mathf::ray;
use rust_ray_tracer::mathf::sphere;
use rust_ray_tracer::mathf::vector3;
use rust_ray_tracer::ppm;
use rust_ray_tracer::mathf::intersection;


fn main() {
    // draw_simple();
    // draw_clock();
    draw_circle();
}

fn draw_circle() {
    let wall_z = 10.0;
    let wall_size = 7.0;
    let canvas_pixels = 100;
    let pixel_size = wall_size / (canvas_pixels as f64);
    let half = wall_size / 2.0;

    let mut canvas = canvas::new(canvas_pixels, canvas_pixels);
    let color = color::red();

    let shape = sphere::new();
    let t = matrix::scaling(&vector3::new(0.5, 1.0, 1.0));
    let shape = shape.set_transform(t);
    let shape = Rc::new(shape);

    for y in 0..canvas_pixels {
        let world_y = half - pixel_size * (y as f64);
        for x in 0..canvas_pixels {
            let world_x = -half + pixel_size * (x as f64);

            let position = vector3::new(world_x, world_y, wall_z);
            let ray_origin = vector3::new(0.0, 0.0, -5.0);
            let ray_origin2 = vector3::new(0.0, 0.0, -5.0);
            let r = ray::new(ray_origin, position.subtract(&ray_origin2));
            let xs = r.intersect(Rc::clone(&shape));

            let xs = intersection::new_intersections(xs);
            let hit = ray::hit(&xs);
            if hit.is_some() {
                canvas.write_pixel(x as isize, y as isize, &color);
            }
        }
    }

    let ppm_data = ppm::canvas_to_ppm(&canvas);
    fs::write("renders/circle.ppm", ppm_data).expect("Unable to write file");

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

#[allow(dead_code)]
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
