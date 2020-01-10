use rust_ray_tracer::camera::Camera;
use rust_ray_tracer::canvas::Canvas;
use rust_ray_tracer::color;
use rust_ray_tracer::color::Color;
use rust_ray_tracer::material::Material;
use rust_ray_tracer::mathf;
use rust_ray_tracer::mathf::intersection::Intersections;
use rust_ray_tracer::mathf::plane::Plane;
use rust_ray_tracer::mathf::ray::Ray;
use rust_ray_tracer::mathf::sphere::Sphere;
use rust_ray_tracer::mathf::vector3::Vector3;
use rust_ray_tracer::phong_lighting;
use rust_ray_tracer::point_light::PointLight;
use rust_ray_tracer::ppm;
use rust_ray_tracer::transformations;
use rust_ray_tracer::world;
use std::f64::consts::PI;
use std::fs;
use std::sync::Arc;
use crate::mathf::shapes::Shape;

fn main() {
    // draw_simple();
    // draw_clock();
    // draw_circle();
    // draw_circle_lit();
    // draw_three_spheres_scene();
    draw_three_spheres_and_plane_scene();
}

#[allow(dead_code)]
fn draw_three_spheres_and_plane_scene() {
    let floor_plane = Plane::new(None, None);


    let middle_transform = transformations::translation(&Vector3::new(-0.5, 1., 0.5));
    let mut middle_material = Material::new();
    middle_material.color = Color::new(0.1, 1., 0.5);
    middle_material.diffuse = 0.7;
    middle_material.specular = 0.3;
    let middle = Sphere::new(Some(middle_transform), Some(middle_material));


    let right_transform = transformations::translation(&Vector3::new(1.5, 0.5, -0.5)).multiply_4x4(&transformations::scaling(&Vector3::new(0.5, 0.5, 0.5)));
    let mut right_material = Material::new();
    right_material.color = Color::new(0.5, 1., 0.1);
    right_material.diffuse = 0.7;
    right_material.specular = 0.3;
    let right = Sphere::new(Some(right_transform), Some(right_material));


    let left_transform = transformations::translation(&Vector3::new(-1.5, 0.33, -0.75)).multiply_4x4(&transformations::scaling(&Vector3::new(0.33, 0.33, 0.33)));
    let mut left_material = Material::new();
    left_material.color = Color::new(1.0, 0.8, 0.1);
    left_material.diffuse = 0.7;
    left_material.specular = 0.3;
    let left = Sphere::new(Some(left_transform), Some(left_material));


    let mut world = world::new();
    let light = PointLight::new(Vector3::new(-10., 10., -10.), color::WHITE);
    world.light = Some(light);
    world.objects = vec![Arc::new(floor_plane), Arc::new(middle), Arc::new(right), Arc::new(left)];

    // let mut camera = Camera::new(100, 50, PI / 3.);
    let mut camera = Camera::new(700, 500, PI / 3.);
    camera.transform = transformations::view_transform(
        Vector3::new(0., 1.5, -5.),
        Vector3::new(0., 1., 0.),
        Vector3::new(0., 1., 0.),
    );

    // let canvas = camera.render(&world);
    let canvas = camera.render_multithreaded(&world);
    let ppm_data = ppm::canvas_to_ppm(&canvas);
    fs::write("renders/three_spheres_and_plane.ppm", ppm_data).expect("Unable to write file");
}


#[allow(dead_code)]
fn draw_three_spheres_scene() {
    let floor_transform = transformations::scaling(&Vector3::new(10., 0.01, 10.));
    let mut floor_material = Material::new();
    floor_material.color = Color::new(1.0, 0.9, 0.9);
    floor_material.specular = 0.;
    let floor = Sphere::new(Some(floor_transform), Some(floor_material));


    let wall_left_transform = transformations::translation(&Vector3::new(0., 0.0, 5.))
        .multiply_4x4(&transformations::rotation_y(-PI / 4.))
        .multiply_4x4(&transformations::rotation_x(PI / 2.))
        .multiply_4x4(&transformations::scaling(&Vector3::new(10., 0.01, 10.)));
    let mut wall_left_material = Material::new();
    wall_left_material.color = Color::new(1.0, 0.9, 0.9);
    wall_left_material.specular = 0.;
    let wall_left = Sphere::new(Some(wall_left_transform), Some(wall_left_material));


    let wall_right_transform = transformations::translation(&Vector3::new(0., 0.0, 5.))
        .multiply_4x4(&transformations::rotation_y(PI / 4.))
        .multiply_4x4(&transformations::rotation_x(PI / 2.))
        .multiply_4x4(&transformations::scaling(&Vector3::new(10., 0.01, 10.)));

    let mut wall_right_material = Material::new();
    wall_right_material.color = Color::new(1.0, 0.9, 0.9);
    wall_right_material.specular = 0.;
    let wall_right = Sphere::new(Some(wall_right_transform), Some(wall_right_material));


    let middle_transform = transformations::translation(&Vector3::new(-0.5, 1., 0.5));
    let mut middle_material = Material::new();
    middle_material.color = Color::new(0.1, 1., 0.5);
    middle_material.diffuse = 0.7;
    middle_material.specular = 0.3;
    let middle = Sphere::new(Some(middle_transform), Some(middle_material));


    let right_transform = transformations::translation(&Vector3::new(1.5, 0.5, -0.5)).multiply_4x4(&transformations::scaling(&Vector3::new(0.5, 0.5, 0.5)));
    let mut right_material = Material::new();
    right_material.color = Color::new(0.5, 1., 0.1);
    right_material.diffuse = 0.7;
    right_material.specular = 0.3;
    let right = Sphere::new(Some(right_transform), Some(right_material));


    let left_transform = transformations::translation(&Vector3::new(-1.5, 0.33, -0.75)).multiply_4x4(&transformations::scaling(&Vector3::new(0.33, 0.33, 0.33)));
    let mut left_material = Material::new();
    left_material.color = Color::new(1.0, 0.8, 0.1);
    left_material.diffuse = 0.7;
    left_material.specular = 0.3;
    let left = Sphere::new(Some(left_transform), Some(left_material));


    let mut world = world::new();
    let light = PointLight::new(Vector3::new(-10., 10., -10.), color::WHITE);
    world.light = Some(light);
    world.objects = vec![Arc::new(floor), Arc::new(wall_left), Arc::new(wall_right), Arc::new(middle), Arc::new(right), Arc::new(left)];

    // let mut camera = Camera::new(100, 50, PI / 3.);
    let mut camera = Camera::new(700, 500, PI / 3.);
    camera.transform = transformations::view_transform(
        Vector3::new(0., 1.5, -5.),
        Vector3::new(0., 1., 0.),
        Vector3::new(0., 1., 0.),
    );

    // let canvas = camera.render(&world);
    let canvas = camera.render_multithreaded(&world);
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

    let shape = Sphere::new(None, Some(material));
    let shape: Arc<dyn Shape> = Arc::new(shape);

    let light_position = Vector3::new(-10.0, 10.0, -10.0);
    let light_color = Color::new(1.0, 1.0, 1.0);
    let light = PointLight::new(light_position, light_color);

    for y in 0..canvas_pixels {
        let world_y = half - pixel_size * (y as f64);
        for x in 0..canvas_pixels {
            let world_x = -half + pixel_size * (x as f64);

            let position = Vector3::new(world_x, world_y, wall_z);

            let ray_origin = Vector3::new(0.0, 0.0, -5.0);
            let ray_origin2 = Vector3::new(0.0, 0.0, -5.0);
            let ray = Ray::new(ray_origin, (&position - &ray_origin2).normalize());
            let xs = shape.intersect(Arc::clone(&shape), ray.clone());
            let xs = Intersections::new(xs);
            let hit = xs.hit();

            if let Some(hit_info) = hit {
                let point = ray.position(hit_info.t);
                let normal = hit_info.object.normal_at(point.clone());
                let eye = -ray.direction;
                let color = phong_lighting::lighting(
                    &hit_info.object.material(),
                    &light,
                    &point,
                    &eye,
                    &normal,
                    false,
                );

                canvas.write_pixel(x as usize, y as usize, &color);
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


    let t = transformations::scaling(&Vector3::new(0.5, 1.0, 1.0));
    let shape = Sphere::new(Some(t), None);
    let shape: Arc<dyn Shape> = Arc::new(shape);

    for y in 0..canvas_pixels {
        let world_y = half - pixel_size * (y as f64);
        for x in 0..canvas_pixels {
            let world_x = -half + pixel_size * (x as f64);

            let position = Vector3::new(world_x, world_y, wall_z);
            let ray_origin = Vector3::new(0.0, 0.0, -5.0);
            let ray_origin2 = Vector3::new(0.0, 0.0, -5.0);
            let ray = Ray::new(ray_origin, &position - &ray_origin2);
            let xs = shape.intersect(Arc::clone(&shape), ray);
            let xs = Intersections::new(xs);
            let hit = xs.hit();
            if hit.is_some() {
                canvas.write_pixel(x as usize, y as usize, &color);
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

    let origin = Vector3::new(0.0, 0.0, 0.0);

    // Draw a dot for every hour on a clock
    for x in 0..12 {
        let to_center_of_canvas = Vector3::new(50.0, 50.0, 0.0);
        let rotation_degrees = (x as f64) * (360.0 / 12.0);

        let rotation_mat = transformations::rotation_z(mathf::degree_to_radian(rotation_degrees));
        let translation_mat = transformations::translation(&Vector3::new(radius, radius, 0.0));

        let position = rotation_mat
            .multiply_4x4(&translation_mat)
            .multiply_point(&origin);
        let position = &position + &to_center_of_canvas;

        canvas.write_pixel(position.y as usize, position.x as usize, &red);
    }

    let ppm_data = ppm::canvas_to_ppm(&canvas);
    fs::write("renders/clock.ppm", ppm_data).expect("Unable to write file");
}
