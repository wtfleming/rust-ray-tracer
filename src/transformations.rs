use crate::mathf::matrix;
use crate::mathf::matrix::Matrix;
use crate::mathf::vector3;
use crate::mathf::vector3::Vector3;

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

    orientation.multiply_4x4(&matrix::translation(&vector3::new(
        -from.x, -from.y, -from.z,
    )))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mathf::approximately;

    #[test]
    fn test_the_transformation_matrix_for_the_default_orientation() {
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
        assert_eq!(transform, matrix::scaling(&vector3::new(-1., 1., -1.)));
    }

    #[test]
    fn a_view_transformation_moves_the_world() {
        let from = vector3::new(0., 0., 8.);
        let to = vector3::new(0., 0., 0.);
        let up = vector3::new(0., 1., 0.);
        let transform = view_transform(&from, &to, &up);
        assert_eq!(transform, matrix::translation(&vector3::new(0., 0., -8.)));
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
