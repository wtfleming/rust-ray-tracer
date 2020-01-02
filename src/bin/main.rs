use std::fs;
use std::rc::Rc;

use rust_ray_tracer::canvas;
use rust_ray_tracer::color;
use rust_ray_tracer::material;
use rust_ray_tracer::mathf;
use rust_ray_tracer::mathf::intersection;
use rust_ray_tracer::mathf::matrix;
use rust_ray_tracer::mathf::ray;
use rust_ray_tracer::mathf::sphere;
use rust_ray_tracer::mathf::vector3;
use rust_ray_tracer::phong_lighting;
use rust_ray_tracer::point_light;
use rust_ray_tracer::ppm;

fn main() {
    // draw_simple();
    // draw_clock();
    // draw_circle();
    draw_circle_lit();
}

fn draw_circle_lit() {
    let wall_z = 10.0;
    let wall_size = 7.0;
    let canvas_pixels = 100;
    let pixel_size = wall_size / (canvas_pixels as f64);
    let half = wall_size / 2.0;

    let mut canvas = canvas::new(canvas_pixels, canvas_pixels);

    let mut material = material::new();
    material.color = color::new(1.0, 0.2, 1.0);

    let shape = sphere::new();
    let shape = shape.set_material(material);
    let shape = Rc::new(shape);

    let light_position = vector3::new(-10.0, 10.0, -10.0);
    let light_color = color::new(1.0, 1.0, 1.0);
    let light = point_light::new(light_position, light_color);

    for y in 0..canvas_pixels {
        let world_y = half - pixel_size * (y as f64);
        for x in 0..canvas_pixels {
            let world_x = -half + pixel_size * (x as f64);

            let position = vector3::new(world_x, world_y, wall_z);

            let ray_origin = vector3::new(0.0, 0.0, -5.0);
            let ray_origin2 = vector3::new(0.0, 0.0, -5.0);
            let r = ray::new(ray_origin, position.subtract(&ray_origin2).normalize());
            let xs = r.intersect(Rc::clone(&shape));

            let xs = intersection::new_intersections(xs);
            let hit = ray::hit(&xs);

            if let Some(hit_info) = hit {
                let point = r.position(hit_info.t);
                let normal = hit_info.object.normal_at(&point);
                let eye = r.direction.negate();
                let color = phong_lighting::lighting(
                    &hit_info.object.material,
                    &light,
                    &point,
                    &eye,
                    &normal,
                );

                canvas.write_pixel(x as isize, y as isize, &color);
            }
        }
    }

    let ppm_data = ppm::canvas_to_ppm(&canvas);
    fs::write("renders/circle_lit.ppm", ppm_data).expect("Unable to write file");
}

#[allow(dead_code)]
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
