use wasm_bindgen::prelude::*;
use web_sys::console;

use std::f64;
use wasm_bindgen::Clamped;

use web_sys::{CanvasRenderingContext2d, ImageData};

use ray_tracer_lib::camera::Camera;
use ray_tracer_lib::color;
use ray_tracer_lib::color::Color;
use ray_tracer_lib::material::Material;
use ray_tracer_lib::mathf::plane::Plane;
use ray_tracer_lib::mathf::sphere::Sphere;
use ray_tracer_lib::mathf::vector3::Vector3;
use ray_tracer_lib::point_light::PointLight;
use ray_tracer_lib::transformations;
use ray_tracer_lib::world;
use std::f64::consts::PI;
use std::sync::Arc;


// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    console::log_1(&JsValue::from_str("Starting!"));

    Ok(())
}


// Convert a color value in the range 0.0 to 1.0 into 0 to 255
fn convert_rbg_value_to_byte(value: f64) -> u8 {
    // Values can be above 1.0, if so clamp them to the correct range for output
    let clamped = num::clamp(value, 0.0, 1.0);
    (clamped * 255.0).ceil() as u8
}


#[wasm_bindgen]
pub fn draw(
    ctx: &CanvasRenderingContext2d,
    width: u32,
    height: u32,
) -> Result<(), JsValue> {

    console::log_1(&JsValue::from_str("Running!"));

    let colors = draw_three_spheres_and_plane_scene(width as usize, height as usize);

    let mut data = Vec::new();
    for row in colors.iter() {
        for column in row.iter() {
            data.push(convert_rbg_value_to_byte(column.r));
            data.push(convert_rbg_value_to_byte(column.g));
            data.push(convert_rbg_value_to_byte(column.b));
            data.push(255);
        }
    }
    console::log_1(&JsValue::from_str("Finished ray tracing!"));

    let data = ImageData::new_with_u8_clamped_array_and_sh(Clamped(&mut data), width, height)?;
    ctx.put_image_data(&data, 0.0, 0.0)
}



fn draw_three_spheres_and_plane_scene(width: usize, height: usize) -> Vec<std::vec::Vec<Color>> {
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

    let mut camera = Camera::new(width, height, PI / 3.);
    camera.transform = transformations::view_transform(
        Vector3::new(0., 1.5, -5.),
        Vector3::new(0., 1., 0.),
        Vector3::new(0., 1., 0.),
    );

    let canvas = camera.render(&world);

    canvas.pixels
}
