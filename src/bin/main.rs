use rust_ray_tracer::camera::Camera;
use rust_ray_tracer::canvas::Canvas;
use rust_ray_tracer::color;
use rust_ray_tracer::color::Color;
use rust_ray_tracer::material::Material;
use rust_ray_tracer::mathf;
use rust_ray_tracer::mathf::intersection::Intersections;
use rust_ray_tracer::mathf::ray::Ray;
use rust_ray_tracer::mathf::sphere;
use rust_ray_tracer::mathf::vector3;
use rust_ray_tracer::phong_lighting;
use rust_ray_tracer::point_light::PointLight;
use rust_ray_tracer::ppm;
use rust_ray_tracer::transformations;
use rust_ray_tracer::world;
use std::f64::consts::PI;
use std::fs;
use std::rc::Rc;

fn main() {
    // draw_simple();
    // draw_clock();
    // draw_circle();
    //draw_circle_lit();
    draw_three_spheres_scene();
}

#[allow(dead_code)]
fn draw_three_spheres_scene() {
    let mut floor = sphere::new();
    floor.transform = transformations::scaling(&vector3::new(10., 0.01, 10.));
    floor.material = Material::new();
    floor.material.color = Color::new(1.0, 0.9, 0.9);
    floor.material.specular = 0.;

    let mut wall_left = sphere::new();
    wall_left.transform = transformations::translation(&vector3::new(0., 0.0, 5.))
        .multiply_4x4(&transformations::rotation_y(-PI / 4.))
        .multiply_4x4(&transformations::rotation_x(PI / 2.))
        .multiply_4x4(&transformations::scaling(&vector3::new(10., 0.01, 10.)));

    wall_left.material = Material::new();
    wall_left.material.color = Color::new(1.0, 0.9, 0.9);
    wall_left.material.specular = 0.;


    let mut wall_right = sphere::new();
    wall_right.transform = transformations::translation(&vector3::new(0., 0.0, 5.))
        .multiply_4x4(&transformations::rotation_y(PI / 4.))
        .multiply_4x4(&transformations::rotation_x(PI / 2.))
        .multiply_4x4(&transformations::scaling(&vector3::new(10., 0.01, 10.)));

    wall_right.material = Material::new();
    wall_right.material.color = Color::new(1.0, 0.9, 0.9);
    wall_right.material.specular = 0.;


    let mut middle = sphere::new();
    middle.transform = transformations::translation(&vector3::new(-0.5, 1., 0.5));
    middle.material = Material::new();
    middle.material.color = Color::new(0.1, 1., 0.5);
    middle.material.diffuse = 0.7;
    middle.material.specular = 0.3;

    let mut right = sphere::new();
    right.transform = transformations::translation(&vector3::new(1.5, 0.5, -0.5)).multiply_4x4(&transformations::scaling(&vector3::new(0.5, 0.5, 0.5)));
    right.material = Material::new();
    right.material.color = Color::new(0.5, 1., 0.1);
    right.material.diffuse = 0.7;
    right.material.specular = 0.3;

    let mut left = sphere::new();
    left.transform = transformations::translation(&vector3::new(-1.5, 0.33, -0.75)).multiply_4x4(&transformations::scaling(&vector3::new(0.33, 0.33, 0.33)));
    left.material = Material::new();
    left.material.color = Color::new(1.0, 0.8, 0.1);
    left.material.diffuse = 0.7;
    left.material.specular = 0.3;


    let mut world = world::new();
    let light = PointLight::new(vector3::new(-10., 10., -10.), color::WHITE);
    world.light = Some(light);
    world.objects = vec![Rc::new(floor), Rc::new(wall_left), Rc::new(wall_right), Rc::new(middle), Rc::new(right), Rc::new(left)];

    let mut camera = Camera::new(100, 50, PI / 3.);
    //let mut camera = Camera::new(700, 500, PI / 3.);
    camera.transform = transformations::view_transform(
        &vector3::new(0., 1.5, -5.),
        &vector3::new(0., 1., 0.),
        &vector3::new(0., 1., 0.),
    );

    let canvas = camera.render(&world);
    let ppm_data = ppm::canvas_to_ppm(&canvas);
    fs::write("renders/three_spheres.ppm", ppm_data).expect("Unable to write file");
}


#[allow(dead_code)]
fn draw_circle_lit() {
    let wall_z = 10.0;
    let wall_size = 7.0;
    let canvas_pixels = 100;
    let pixel_size = wall_size / (canvas_pixels as f64);
    let half = wall_size / 2.0;

    let mut canvas = Canvas::new(canvas_pixels, canvas_pixels);

    let mut material = Material::new();
    material.color = Color::new(1.0, 0.2, 1.0);

    let mut shape = sphere::new();
    shape.material = material;
    let shape = Rc::new(shape);

    let light_position = vector3::new(-10.0, 10.0, -10.0);
    let light_color = Color::new(1.0, 1.0, 1.0);
    let light = PointLight::new(light_position, light_color);

    for y in 0..canvas_pixels {
        let world_y = half - pixel_size * (y as f64);
        for x in 0..canvas_pixels {
            let world_x = -half + pixel_size * (x as f64);

            let position = vector3::new(world_x, world_y, wall_z);

            let ray_origin = vector3::new(0.0, 0.0, -5.0);
            let ray_origin2 = vector3::new(0.0, 0.0, -5.0);
            let r = Ray::new(ray_origin, (&position - &ray_origin2).normalize());
            let xs = r.intersect(Rc::clone(&shape));

            let xs = Intersections::new(xs);
            let hit = xs.hit();

            if let Some(hit_info) = hit {
                let point = r.position(hit_info.t);
                let normal = hit_info.object.normal_at(&point);
                let eye = -r.direction;
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

    let mut canvas = Canvas::new(canvas_pixels, canvas_pixels);
    let color = color::RED;

    let mut shape = sphere::new();
    let t = transformations::scaling(&vector3::new(0.5, 1.0, 1.0));
    shape.transform = t;
    let shape = Rc::new(shape);

    for y in 0..canvas_pixels {
        let world_y = half - pixel_size * (y as f64);
        for x in 0..canvas_pixels {
            let world_x = -half + pixel_size * (x as f64);

            let position = vector3::new(world_x, world_y, wall_z);
            let ray_origin = vector3::new(0.0, 0.0, -5.0);
            let ray_origin2 = vector3::new(0.0, 0.0, -5.0);
            let r = Ray::new(ray_origin, &position - &ray_origin2);
            let xs = r.intersect(Rc::clone(&shape));

            let xs = Intersections::new(xs);
            let hit = xs.hit();
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
    let mut canvas = Canvas::new(5, 3);
    let red = color::RED;
    let green = color::GREEN;
    let blue = color::BLUE;
    canvas.write_pixel(0, 0, &red);
    canvas.write_pixel(2, 1, &green);
    canvas.write_pixel(4, 2, &blue);

    let ppm_data = ppm::canvas_to_ppm(&canvas);
    fs::write("renders/output.ppm", ppm_data).expect("Unable to write file");
}

#[allow(dead_code)]
fn draw_clock() {
    let mut canvas = Canvas::new(100, 100);
    let radius = 30.0;
    let red = color::RED;

    let origin = vector3::new(0.0, 0.0, 0.0);

    // Draw a dot for every hour on a clock
    for x in 0..12 {
        let to_center_of_canvas = vector3::new(50.0, 50.0, 0.0);
        let rotation_degrees = (x as f64) * (360.0 / 12.0);

        let rotation_mat = transformations::rotation_z(mathf::degree_to_radian(rotation_degrees));
        let translation_mat = transformations::translation(&vector3::new(radius, radius, 0.0));

        let position = rotation_mat
            .multiply_4x4(&translation_mat)
            .multiply_vector3(&origin);
        let position = &position + &to_center_of_canvas;

        canvas.write_pixel(position.y as isize, position.x as isize, &red);
    }

    let ppm_data = ppm::canvas_to_ppm(&canvas);
    fs::write("renders/clock.ppm", ppm_data).expect("Unable to write file");
}
