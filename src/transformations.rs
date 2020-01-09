use crate::mathf::matrix::Matrix;
use crate::mathf::matrix::Row;
use crate::mathf::vector3::Vector3;

/// Creates a translation matrix
pub fn translation(vector3: &Vector3) -> Matrix {
    let mut matrix = Matrix::identity_4x4();
    matrix[0] = Row::new(vec![1., 0., 0., vector3.x]);
    matrix[1] = Row::new(vec![0., 1., 0., vector3.y]);
    matrix[2] = Row::new(vec![0., 0., 1., vector3.z]);
    matrix[3] = Row::new(vec![0., 0., 0., 1.]);
    matrix
}

/// Creates a scaling matrix
pub fn scaling(vector3: &Vector3) -> Matrix {
    let mut matrix = Matrix::identity_4x4();
    matrix[0] = Row::new(vec![vector3.x, 0., 0., 0.]);
    matrix[1] = Row::new(vec![0., vector3.y, 0., 0.]);
    matrix[2] = Row::new(vec![0., 0., vector3.z, 0.]);
    matrix[3] = Row::new(vec![0., 0., 0., 1.]);
    matrix
}

/// Creates a rotation around the x axis matrix
pub fn rotation_x(radians: f64) -> Matrix {
    let mut matrix = Matrix::identity_4x4();
    matrix[0] = Row::new(vec![1., 0., 0., 0.]);
    matrix[1] = Row::new(vec![0., radians.cos(), -radians.sin(), 0.]);
    matrix[2] = Row::new(vec![0., radians.sin(), radians.cos(), 0.]);
    matrix[3] = Row::new(vec![0., 0., 0., 1.]);
    matrix
}

/// Creates a rotation around the y axis matrix
pub fn rotation_y(radians: f64) -> Matrix {
    let mut matrix = Matrix::identity_4x4();
    matrix[0] = Row::new(vec![radians.cos(), 0., radians.sin(), 0.]);
    matrix[1] = Row::new(vec![0., 1., 0., 0.]);
    matrix[2] = Row::new(vec![-radians.sin(), 0., radians.cos(), 0.]);
    matrix[3] = Row::new(vec![0., 0., 0., 1.]);
    matrix
}

/// Creates a rotation around the z axis matrix
pub fn rotation_z(radians: f64) -> Matrix {
    let mut matrix = Matrix::identity_4x4();
    matrix[0] = Row::new(vec![radians.cos(), -radians.sin(), 0., 0.]);
    matrix[1] = Row::new(vec![radians.sin(), radians.cos(), 0., 0.]);
    matrix[2] = Row::new(vec![0., 0., 1., 0.]);
    matrix[3] = Row::new(vec![0., 0., 0., 1.]);
    matrix
}

/// Creates a shearing matrix
pub fn shearing(xy: f64, xz: f64, yx: f64, yz: f64, zx: f64, zy: f64) -> Matrix {
    let mut matrix = Matrix::identity_4x4();
    matrix[0] = Row::new(vec![1., xy, xz, 0.]);
    matrix[1] = Row::new(vec![yx, 1., yz, 0.]);
    matrix[2] = Row::new(vec![zx, zy, 1., 0.]);
    matrix[3] = Row::new(vec![0., 0., 0., 1.]);
    matrix
}

pub fn view_transform(from: Vector3, to: Vector3, up: Vector3) -> Matrix {
    let forward = (&to - &from).normalize();
    let left = forward.cross(&up.normalize());
    let true_up = left.cross(&forward);

    let mut orientation = Matrix::identity_4x4();
    orientation.data[0] = Row::new(vec![left.x, left.y, left.z, 0.]);
    orientation.data[1] = Row::new(vec![true_up.x, true_up.y, true_up.z, 0.]);
    orientation.data[2] = Row::new(vec![-forward.x, -forward.y, -forward.z, 0.]);
    orientation.data[3] = Row::new(vec![0., 0., 0., 1.]);

    orientation.multiply_4x4(&translation(&Vector3::new(-from.x, -from.y, -from.z)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mathf::approximately;
    use std::f64::consts::PI;

    #[test]
    fn test_multiplying_by_a_translation_matrix() {
        let transform = translation(&Vector3::new(5.0, -3.0, 2.0));

        let point = Vector3::new(-3.0, 4.0, 5.0);
        let result = transform.multiply_point(&point);
        assert_eq!(result, Vector3::new(2.0, 1.0, 7.0));
    }

    #[test]
    fn test_multiplying_by_the_inverse_of_a_translation_matrix() {
        let transform = translation(&Vector3::new(5.0, -3.0, 2.0));
        let inv = transform.inverse();

        let point = Vector3::new(-3.0, 4.0, 5.0);
        let result = inv.multiply_point(&point);
        assert_eq!(result, Vector3::new(-8.0, 7.0, 3.0));
    }

    #[test]
    fn test_multiplying_by_a_scaling_matrix() {
        let transform = scaling(&Vector3::new(2.0, 3.0, 4.0));

        let point = Vector3::new(-4.0, 6.0, 8.0);
        let result = transform.multiply_point(&point);
        assert_eq!(result, Vector3::new(-8.0, 18.0, 32.0));
    }

    #[test]
    fn test_multiplying_by_the_inverse_of_a_scaling_matrix() {
        let transform = scaling(&Vector3::new(2.0, 3.0, 4.0));
        let inv = transform.inverse();

        let point = Vector3::new(-4.0, 6.0, 8.0);
        let result = inv.multiply_point(&point);
        assert_eq!(result, Vector3::new(-2.0, 2.0, 2.0));
    }

    #[test]
    fn test_reflection_is_scaling_by_a_negative_value() {
        let transform = scaling(&Vector3::new(-1.0, 1.0, 1.0));

        let point = Vector3::new(2.0, 3.0, 4.0);
        let result = transform.multiply_point(&point);
        assert_eq!(result, Vector3::new(-2.0, 3.0, 4.0));
    }

    #[test]
    fn test_rotate_around_x_axis() {
        let point = Vector3::new(0.0, 1.0, 0.0);
        let half_quarter = rotation_x(PI / 4.0);
        let full_quarter = rotation_x(PI / 2.0);

        let half_quarter_expected = Vector3::new(0.0, 2.0f64.sqrt() / 2.0, 2.0f64.sqrt() / 2.0);
        assert_eq!(half_quarter.multiply_point(&point), half_quarter_expected);

        let full_quarter_expected = Vector3::new(0.0, 0.0, 1.0);
        assert_eq!(full_quarter.multiply_point(&point), full_quarter_expected);
    }

    #[test]
    fn test_inverse_of_rotate_around_x_axis_rotates_in_the_opposite_direction() {
        let point = Vector3::new(0.0, 1.0, 0.0);
        let half_quarter = rotation_x(PI / 4.0);

        let inv = half_quarter.inverse();
        let half_quarter_expected = Vector3::new(0.0, 2.0f64.sqrt() / 2.0, -(2.0f64.sqrt() / 2.0));
        assert_eq!(inv.multiply_point(&point), half_quarter_expected);
    }

    #[test]
    fn test_rotate_around_y_axis() {
        let point = Vector3::new(0.0, 0.0, 1.0);
        let half_quarter = rotation_y(PI / 4.0);
        let full_quarter = rotation_y(PI / 2.0);

        let half_quarter_expected = Vector3::new(2.0f64.sqrt() / 2.0, 0.0, 2.0f64.sqrt() / 2.0);
        assert_eq!(half_quarter.multiply_point(&point), half_quarter_expected);

        let full_quarter_expected = Vector3::new(1.0, 0.0, 0.0);
        assert_eq!(full_quarter.multiply_point(&point), full_quarter_expected);
    }

    #[test]
    fn test_rotate_around_z_axis() {
        let point = Vector3::new(0.0, 1.0, 0.0);
        let half_quarter = rotation_z(PI / 4.0);
        let full_quarter = rotation_z(PI / 2.0);

        let half_quarter_expected = Vector3::new(-(2.0f64.sqrt() / 2.0), 2.0f64.sqrt() / 2.0, 0.0);
        assert_eq!(half_quarter.multiply_point(&point), half_quarter_expected);

        let full_quarter_expected = Vector3::new(-1.0, 0.0, 0.0);
        assert_eq!(full_quarter.multiply_point(&point), full_quarter_expected);
    }

    #[test]
    fn a_shearing_transformation_moves_x_in_proportion_to_y() {
        let transform = shearing(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        let point = Vector3::new(2.0, 3.0, 4.0);
        let result = transform.multiply_point(&point);
        assert_eq!(result, Vector3::new(5.0, 3.0, 4.0));
    }

    #[test]
    fn a_shearing_transformation_moves_x_in_proportion_to_z() {
        let transform = shearing(0.0, 1.0, 0.0, 0.0, 0.0, 0.0);
        let point = Vector3::new(2.0, 3.0, 4.0);
        let result = transform.multiply_point(&point);
        assert_eq!(result, Vector3::new(6.0, 3.0, 4.0));
    }

    #[test]
    fn a_shearing_transformation_moves_y_in_proportion_to_x() {
        let transform = shearing(0.0, 0.0, 1.0, 0.0, 0.0, 0.0);
        let point = Vector3::new(2.0, 3.0, 4.0);
        let result = transform.multiply_point(&point);
        assert_eq!(result, Vector3::new(2.0, 5.0, 4.0));
    }

    #[test]
    fn a_shearing_transformation_moves_y_in_proportion_to_z() {
        let transform = shearing(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
        let point = Vector3::new(2.0, 3.0, 4.0);
        let result = transform.multiply_point(&point);
        assert_eq!(result, Vector3::new(2.0, 7.0, 4.0));
    }

    #[test]
    fn a_shearing_transformation_moves_z_in_proportion_to_x() {
        let transform = shearing(0.0, 0.0, 0.0, 0.0, 1.0, 0.0);
        let point = Vector3::new(2.0, 3.0, 4.0);
        let result = transform.multiply_point(&point);
        assert_eq!(result, Vector3::new(2.0, 3.0, 6.0));
    }

    #[test]
    fn a_shearing_transformation_moves_z_in_proportion_to_y() {
        let transform = shearing(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);
        let point = Vector3::new(2.0, 3.0, 4.0);
        let result = transform.multiply_point(&point);
        assert_eq!(result, Vector3::new(2.0, 3.0, 7.0));
    }

    #[test]
    fn individual_transformations_are_applied_in_sequence() {
        let point = Vector3::new(1.0, 0.0, 1.0);
        let rotation = rotation_x(PI / 2.0);
        let scaling = scaling(&Vector3::new(5.0, 5.0, 5.0));
        let translation = translation(&Vector3::new(10.0, 5.0, 7.0));

        let p2 = rotation.multiply_point(&point);
        assert_eq!(p2, Vector3::new(1.0, -1.0, 0.0));

        let p3 = scaling.multiply_point(&p2);
        assert_eq!(p3, Vector3::new(5.0, -5.0, 0.0));

        let p4 = translation.multiply_point(&p3);
        assert_eq!(p4, Vector3::new(15.0, 0.0, 7.0));
    }

    #[test]
    fn chained_transformations_must_be_applied_in_reverse_order() {
        let point = Vector3::new(1.0, 0.0, 1.0);
        let rotation = rotation_x(PI / 2.0);
        let scaling = scaling(&Vector3::new(5.0, 5.0, 5.0));
        let translation = translation(&Vector3::new(10.0, 5.0, 7.0));

        let transform = translation.multiply_4x4(&scaling).multiply_4x4(&rotation);

        let expected = Vector3::new(15.0, 0.0, 7.0);
        assert_eq!(expected, transform.multiply_point(&point));
    }

    #[test]
    fn test_the_view_transformation_matrix_for_the_default_orientation() {
        let from = Vector3::new(0., 0., 0.);
        let to = Vector3::new(0., 0., -1.);
        let up = Vector3::new(0., 1., 0.);
        let transform = view_transform(from, to, up);
        assert_eq!(transform, Matrix::identity_4x4());
    }

    #[test]
    fn a_view_transformation_matrix_looking_in_the_positive_z_direction() {
        let from = Vector3::new(0., 0., 0.);
        let to = Vector3::new(0., 0., 1.);
        let up = Vector3::new(0., 1., 0.);
        let transform = view_transform(from, to, up);
        assert_eq!(transform, scaling(&Vector3::new(-1., 1., -1.)));
    }

    #[test]
    fn a_view_transformation_moves_the_world() {
        let from = Vector3::new(0., 0., 8.);
        let to = Vector3::new(0., 0., 0.);
        let up = Vector3::new(0., 1., 0.);
        let transform = view_transform(from, to, up);
        assert_eq!(transform, translation(&Vector3::new(0., 0., -8.)));
    }

    #[test]
    fn an_arbitrary_view_transformation() {
        let from = Vector3::new(1., 3., 2.);
        let to = Vector3::new(4., -2., 8.);
        let up = Vector3::new(1., 1., 0.);
        let transform = view_transform(from, to, up);

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
