use crate::mathf::matrix;
use crate::mathf::matrix::Matrix;
use crate::mathf::vector3;
use crate::mathf::vector3::Vector3;

/// Creates a translation matrix
pub fn translation(vector3: &Vector3) -> Matrix {
    let mut matrix = matrix::identity_4x4();
    matrix.data[0][3] = vector3.x;
    matrix.data[1][3] = vector3.y;
    matrix.data[2][3] = vector3.z;
    matrix
}

/// Creates a scaling matrix
pub fn scaling(vector3: &Vector3) -> Matrix {
    let mut matrix = matrix::identity_4x4();
    matrix.data[0][0] = vector3.x;
    matrix.data[1][1] = vector3.y;
    matrix.data[2][2] = vector3.z;
    matrix
}

/// Creates a rotation around the x axis matrix
pub fn rotation_x(radians: f64) -> Matrix {
    let mut matrix = matrix::identity_4x4();
    matrix.data[1][1] = radians.cos();
    matrix.data[1][2] = -radians.sin();
    matrix.data[2][1] = radians.sin();
    matrix.data[2][2] = radians.cos();
    matrix
}

/// Creates a rotation around the y axis matrix
pub fn rotation_y(radians: f64) -> Matrix {
    let mut matrix = matrix::identity_4x4();
    matrix.data[0][0] = radians.cos();
    matrix.data[0][2] = radians.sin();
    matrix.data[2][0] = -radians.sin();
    matrix.data[2][2] = radians.cos();
    matrix
}

/// Creates a rotation around the z axis matrix
pub fn rotation_z(radians: f64) -> Matrix {
    let mut matrix = matrix::identity_4x4();
    matrix.data[0][0] = radians.cos();
    matrix.data[0][1] = -radians.sin();
    matrix.data[1][0] = radians.sin();
    matrix.data[1][1] = radians.cos();
    matrix
}

/// Creates a shearing matrix
pub fn shearing(xy: f64, xz: f64, yx: f64, yz: f64, zx: f64, zy: f64) -> Matrix {
    let mut matrix = matrix::identity_4x4();
    matrix.data[0][1] = xy;
    matrix.data[0][2] = xz;
    matrix.data[1][0] = yx;
    matrix.data[1][2] = yz;
    matrix.data[2][0] = zx;
    matrix.data[2][1] = zy;
    matrix
}

pub fn view_transform(from: &Vector3, to: &Vector3, up: &Vector3) -> Matrix {
    let forward = (to - from).normalize();
    let upn = up.normalize();
    let left = forward.cross(&upn);
    let true_up = left.cross(&forward);

    let mut orientation = matrix::identity_4x4();
    orientation.data[0][0] = left.x;
    orientation.data[0][1] = left.y;
    orientation.data[0][2] = left.z;
    orientation.data[0][3] = 0.;

    orientation.data[1][0] = true_up.x;
    orientation.data[1][1] = true_up.y;
    orientation.data[1][2] = true_up.z;
    orientation.data[1][3] = 0.;

    orientation.data[2][0] = -forward.x;
    orientation.data[2][1] = -forward.y;
    orientation.data[2][2] = -forward.z;
    orientation.data[2][3] = 0.;

    orientation.data[3][0] = 0.;
    orientation.data[3][1] = 0.;
    orientation.data[3][2] = 0.;
    orientation.data[3][3] = 1.;

    orientation.multiply_4x4(&translation(&vector3::new(-from.x, -from.y, -from.z)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mathf::approximately;
    use std::f64::consts::PI;

    #[test]
    fn test_multiplying_by_a_translation_matrix() {
        let transform = translation(&vector3::new(5.0, -3.0, 2.0));

        let point = vector3::new(-3.0, 4.0, 5.0);
        let result = transform.multiply_vector3(&point);
        assert_eq!(result, vector3::new(2.0, 1.0, 7.0));
    }

    #[test]
    fn test_multiplying_by_the_inverse_of_a_translation_matrix() {
        let transform = translation(&vector3::new(5.0, -3.0, 2.0));
        let inv = transform.inverse();

        let point = vector3::new(-3.0, 4.0, 5.0);
        let result = inv.multiply_vector3(&point);
        assert_eq!(result, vector3::new(-8.0, 7.0, 3.0));
    }

    #[test]
    fn test_multiplying_by_a_scaling_matrix() {
        let transform = scaling(&vector3::new(2.0, 3.0, 4.0));

        let point = vector3::new(-4.0, 6.0, 8.0);
        let result = transform.multiply_vector3(&point);
        assert_eq!(result, vector3::new(-8.0, 18.0, 32.0));
    }

    #[test]
    fn test_multiplying_by_the_inverse_of_a_scaling_matrix() {
        let transform = scaling(&vector3::new(2.0, 3.0, 4.0));
        let inv = transform.inverse();

        let point = vector3::new(-4.0, 6.0, 8.0);
        let result = inv.multiply_vector3(&point);
        assert_eq!(result, vector3::new(-2.0, 2.0, 2.0));
    }

    #[test]
    fn test_reflection_is_scaling_by_a_negative_value() {
        let transform = scaling(&vector3::new(-1.0, 1.0, 1.0));

        let point = vector3::new(2.0, 3.0, 4.0);
        let result = transform.multiply_vector3(&point);
        assert_eq!(result, vector3::new(-2.0, 3.0, 4.0));
    }

    #[test]
    fn test_rotate_around_x_axis() {
        let point = vector3::new(0.0, 1.0, 0.0);
        let half_quarter = rotation_x(PI / 4.0);
        let full_quarter = rotation_x(PI / 2.0);

        let half_quarter_expected = vector3::new(0.0, 2.0f64.sqrt() / 2.0, 2.0f64.sqrt() / 2.0);
        assert_eq!(half_quarter.multiply_vector3(&point), half_quarter_expected);

        let full_quarter_expected = vector3::new(0.0, 0.0, 1.0);
        assert_eq!(full_quarter.multiply_vector3(&point), full_quarter_expected);
    }

    #[test]
    fn test_inverse_of_rotate_around_x_axis_rotates_in_the_opposite_direction() {
        let point = vector3::new(0.0, 1.0, 0.0);
        let half_quarter = rotation_x(PI / 4.0);

        let inv = half_quarter.inverse();
        let half_quarter_expected = vector3::new(0.0, 2.0f64.sqrt() / 2.0, -(2.0f64.sqrt() / 2.0));
        assert_eq!(inv.multiply_vector3(&point), half_quarter_expected);
    }

    #[test]
    fn test_rotate_around_y_axis() {
        let point = vector3::new(0.0, 0.0, 1.0);
        let half_quarter = rotation_y(PI / 4.0);
        let full_quarter = rotation_y(PI / 2.0);

        let half_quarter_expected = vector3::new(2.0f64.sqrt() / 2.0, 0.0, 2.0f64.sqrt() / 2.0);
        assert_eq!(half_quarter.multiply_vector3(&point), half_quarter_expected);

        let full_quarter_expected = vector3::new(1.0, 0.0, 0.0);
        assert_eq!(full_quarter.multiply_vector3(&point), full_quarter_expected);
    }

    #[test]
    fn test_rotate_around_z_axis() {
        let point = vector3::new(0.0, 1.0, 0.0);
        let half_quarter = rotation_z(PI / 4.0);
        let full_quarter = rotation_z(PI / 2.0);

        let half_quarter_expected = vector3::new(-(2.0f64.sqrt() / 2.0), 2.0f64.sqrt() / 2.0, 0.0);
        assert_eq!(half_quarter.multiply_vector3(&point), half_quarter_expected);

        let full_quarter_expected = vector3::new(-1.0, 0.0, 0.0);
        assert_eq!(full_quarter.multiply_vector3(&point), full_quarter_expected);
    }

    #[test]
    fn a_shearing_transformation_moves_x_in_proportion_to_y() {
        let transform = shearing(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        let point = vector3::new(2.0, 3.0, 4.0);
        let result = transform.multiply_vector3(&point);
        assert_eq!(result, vector3::new(5.0, 3.0, 4.0));
    }

    #[test]
    fn a_shearing_transformation_moves_x_in_proportion_to_z() {
        let transform = shearing(0.0, 1.0, 0.0, 0.0, 0.0, 0.0);
        let point = vector3::new(2.0, 3.0, 4.0);
        let result = transform.multiply_vector3(&point);
        assert_eq!(result, vector3::new(6.0, 3.0, 4.0));
    }

    #[test]
    fn a_shearing_transformation_moves_y_in_proportion_to_x() {
        let transform = shearing(0.0, 0.0, 1.0, 0.0, 0.0, 0.0);
        let point = vector3::new(2.0, 3.0, 4.0);
        let result = transform.multiply_vector3(&point);
        assert_eq!(result, vector3::new(2.0, 5.0, 4.0));
    }

    #[test]
    fn a_shearing_transformation_moves_y_in_proportion_to_z() {
        let transform = shearing(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
        let point = vector3::new(2.0, 3.0, 4.0);
        let result = transform.multiply_vector3(&point);
        assert_eq!(result, vector3::new(2.0, 7.0, 4.0));
    }

    #[test]
    fn a_shearing_transformation_moves_z_in_proportion_to_x() {
        let transform = shearing(0.0, 0.0, 0.0, 0.0, 1.0, 0.0);
        let point = vector3::new(2.0, 3.0, 4.0);
        let result = transform.multiply_vector3(&point);
        assert_eq!(result, vector3::new(2.0, 3.0, 6.0));
    }

    #[test]
    fn a_shearing_transformation_moves_z_in_proportion_to_y() {
        let transform = shearing(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);
        let point = vector3::new(2.0, 3.0, 4.0);
        let result = transform.multiply_vector3(&point);
        assert_eq!(result, vector3::new(2.0, 3.0, 7.0));
    }

    #[test]
    fn individual_transformations_are_applied_in_sequence() {
        let point = vector3::new(1.0, 0.0, 1.0);
        let rotation = rotation_x(PI / 2.0);
        let scaling = scaling(&vector3::new(5.0, 5.0, 5.0));
        let translation = translation(&vector3::new(10.0, 5.0, 7.0));

        let p2 = rotation.multiply_vector3(&point);
        assert_eq!(p2, vector3::new(1.0, -1.0, 0.0));

        let p3 = scaling.multiply_vector3(&p2);
        assert_eq!(p3, vector3::new(5.0, -5.0, 0.0));

        let p4 = translation.multiply_vector3(&p3);
        assert_eq!(p4, vector3::new(15.0, 0.0, 7.0));
    }

    #[test]
    fn chained_transformations_must_be_applied_in_reverse_order() {
        let point = vector3::new(1.0, 0.0, 1.0);
        let rotation = rotation_x(PI / 2.0);
        let scaling = scaling(&vector3::new(5.0, 5.0, 5.0));
        let translation = translation(&vector3::new(10.0, 5.0, 7.0));

        let transform = translation.multiply_4x4(&scaling).multiply_4x4(&rotation);

        let expected = vector3::new(15.0, 0.0, 7.0);
        assert_eq!(expected, transform.multiply_vector3(&point));
    }

    #[test]
    fn test_the_view_transformation_matrix_for_the_default_orientation() {
        let from = vector3::new(0., 0., 0.);
        let to = vector3::new(0., 0., -1.);
        let up = vector3::new(0., 1., 0.);
        let transform = view_transform(&from, &to, &up);
        assert_eq!(transform, matrix::identity_4x4());
    }

    #[test]
    fn a_view_transformation_matrix_looking_in_the_positive_z_direction() {
        let from = vector3::new(0., 0., 0.);
        let to = vector3::new(0., 0., 1.);
        let up = vector3::new(0., 1., 0.);
        let transform = view_transform(&from, &to, &up);
        assert_eq!(transform, scaling(&vector3::new(-1., 1., -1.)));
    }

    #[test]
    fn a_view_transformation_moves_the_world() {
        let from = vector3::new(0., 0., 8.);
        let to = vector3::new(0., 0., 0.);
        let up = vector3::new(0., 1., 0.);
        let transform = view_transform(&from, &to, &up);
        assert_eq!(transform, translation(&vector3::new(0., 0., -8.)));
    }

    #[test]
    fn an_arbitrary_view_transformation() {
        let from = vector3::new(1., 3., 2.);
        let to = vector3::new(4., -2., 8.);
        let up = vector3::new(1., 1., 0.);
        let transform = view_transform(&from, &to, &up);
        //        assert_eq!(transform, matrix::translation(&vector3::new(0., 0., -8.)));

        assert!(approximately(transform.data[0][0], -0.50709));
        assert!(approximately(transform.data[0][1], 0.50709));
        assert!(approximately(transform.data[0][2], 0.67612));
        assert!(approximately(transform.data[0][3], -2.36643));

        assert!(approximately(transform.data[1][0], 0.76772));
        assert!(approximately(transform.data[1][1], 0.60609));
        assert!(approximately(transform.data[1][2], 0.12122));
        assert!(approximately(transform.data[1][3], -2.82843));

        assert!(approximately(transform.data[2][0], -0.35857));
        assert!(approximately(transform.data[2][1], 0.59761));
        assert!(approximately(transform.data[2][2], -0.71714));
        assert!(approximately(transform.data[2][3], 0.00000));

        assert!(approximately(transform.data[3][0], 0.00000));
        assert!(approximately(transform.data[3][1], 0.00000));
        assert!(approximately(transform.data[3][2], 0.00000));
        assert!(approximately(transform.data[3][3], 1.00000));
    }
}
