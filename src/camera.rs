use crate::canvas::Canvas;
use crate::mathf::matrix::Matrix;
use crate::mathf::ray::Ray;
use crate::mathf::vector3::Vector3;
use crate::world::World;

pub struct Camera {
    pub hsize: usize,
    pub vsize: usize,
    pub field_of_view: f64,
    pub transform: Matrix,
    pub pixel_size: f64,
    pub half_width: f64,
    pub half_height: f64,
}

impl Camera {
    pub fn new(hsize: usize, vsize: usize, field_of_view: f64) -> Camera {
        let half_view = (field_of_view / 2.).tan();
        let aspect_ratio = hsize as f64 / vsize as f64;

        let half_width;
        let half_height;
        if aspect_ratio >= 1. {
            half_width = half_view;
            half_height = half_view / aspect_ratio;
        } else {
            half_width = half_view * aspect_ratio;
            half_height = half_view;
        }

        let pixel_size = half_width * 2. / hsize as f64;

        Camera {
            hsize,
            vsize,
            field_of_view,
            transform: Matrix::identity_4x4(),
            pixel_size,
            half_width,
            half_height,
        }
    }

    pub fn ray_for_pixel(&self, px: usize, py: usize) -> Ray {
        // The offset from the edge of the canvas to the pixel's center
        let xoffset = (px as f64 + 0.5) * self.pixel_size;
        let yoffset = (py as f64 + 0.5) * self.pixel_size;

        // The untransformed coordinates of the pixel in world space.
        // Remember the camera looks toward -z, so +x is to the *left*
        let world_x = self.half_width - xoffset;
        let world_y = self.half_height - yoffset;

        // Using the camera matrix, transform the canvas point and the origin,
        // and then compute the ray's direction vector.
        // Remember that the canvas is at z=-1
        let pixel = self
            .transform
            .inverse()
            .multiply_vector3(&Vector3::new(world_x, world_y, -1.));
        let origin = self
            .transform
            .inverse()
            .multiply_vector3(&Vector3::new(0., 0., 0.));

        let direction = (&pixel - &origin).normalize();
        Ray::new(origin, direction)
    }

    pub fn render(&self, world: &World) -> Canvas {
        let mut image = Canvas::new(self.hsize, self.vsize);
        for y in 0..self.vsize {
            for x in 0..self.hsize {
                let ray = self.ray_for_pixel(x, y);
                let color = world.color_at(ray); // MUST BE HAPPENING IN THIS FN, THE RAYS LOOK OK?           
                image.write_pixel(x as isize, y as isize, &color);
            }
        }
        image
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color::Color;
    use crate::mathf::approximately;
    use crate::transformations;
    use crate::world;
    use std::f64::consts::PI;

    #[test]
    fn it_creates_a_camera() {
        let hsize = 160;
        let vsize = 120;
        let field_of_view = PI / 2.;
        let camera = Camera::new(hsize, vsize, field_of_view);
        assert_eq!(camera.hsize, 160);
        assert_eq!(camera.vsize, 120);
        assert_eq!(camera.field_of_view, PI / 2.);
        assert_eq!(camera.transform, Matrix::identity_4x4());
    }

    #[test]
    fn the_pixel_size_for_a_horizontal_camera() {
        let camera = Camera::new(200, 125, PI / 2.);
        assert!(approximately(camera.pixel_size, 0.01));
    }

    #[test]
    fn the_pixel_size_for_a_vertical_camera() {
        let camera = Camera::new(125, 200, PI / 2.);
        assert!(approximately(camera.pixel_size, 0.01));
    }

    #[test]
    fn constructing_a_ray_through_the_center_of_the_canvas() {
        let camera = Camera::new(201, 101, PI / 2.);
        let ray = camera.ray_for_pixel(100, 50);
        assert_eq!(ray.origin, Vector3::new(0., 0., 0.));
        assert_eq!(ray.direction, Vector3::new(0., 0., -1.));
    }

    #[test]
    fn constructing_a_ray_through_a_corner_of_the_canvas() {
        let camera = Camera::new(201, 101, PI / 2.);
        let ray = camera.ray_for_pixel(0, 0);
        assert_eq!(ray.origin, Vector3::new(0., 0., 0.));
        assert_eq!(ray.direction, Vector3::new(0.66519, 0.33259, -0.66851));
    }

    #[test]
    fn constructing_a_ray_when_the_camera_is_transformed() {
        let mut camera = Camera::new(201, 101, PI / 2.);
        camera.transform = transformations::rotation_y(PI / 4.)
            .multiply_4x4(&transformations::translation(&Vector3::new(0., -2., 5.)));

        let ray = camera.ray_for_pixel(100, 50);
        assert_eq!(ray.origin, Vector3::new(0., 2., -5.));
        assert_eq!(
            ray.direction,
            Vector3::new(2.0f64.sqrt() / 2., 0., -(2.0f64.sqrt() / 2.))
        );
    }

    #[test]
    fn rendering_a_world_with_a_camera() {
        let world = world::default_world();
        let mut camera = Camera::new(11, 11, PI / 2.);
        let from = Vector3::new(0., 0., -5.);
        let to = Vector3::new(0., 0., 0.);
        let up = Vector3::new(0., 1., 0.);
        camera.transform = transformations::view_transform(from, to, up);
        let image = camera.render(&world);

        let pixel_at = &image.pixels[5][5];
        assert_eq!(pixel_at, &Color::new(0.38066, 0.47583, 0.2855));
    }
}
