use crate::mathf;
use crate::mathf::vector3::Vector3;

#[derive(Debug, Clone)]
pub struct Matrix {
    pub size: usize,
    pub data: [Row; 4],
}

#[derive(Debug, Clone, Copy)]
pub struct Row {
    columns: [f64; 4],
}

// ------------ Row implementations ------------
impl Row {
    pub fn new(data: [f64; 4]) -> Row {
        Row { columns: data }
    }
}

impl std::ops::Index<usize> for Row {
    type Output = f64;
    fn index(&self, col: usize) -> &Self::Output {
        &self.columns[col]
    }
}
impl std::ops::IndexMut<usize> for Row {
    fn index_mut(&mut self, col: usize) -> &mut Self::Output {
        &mut self.columns[col]
    }
}

impl PartialEq for Row {
    fn eq(&self, other: &Self) -> bool {
        (0..self.columns.len()).all(|col| mathf::approximately(self[col], other[col]))
    }
}

// ------------ Matrix implementations ------------
impl PartialEq for Matrix {
    fn eq(&self, other: &Self) -> bool {
        for r in 0..self.size {
            for c in 0..self.size {
                if !mathf::approximately(self.data[r][c], other.data[r][c]) {
                    return false;
                }
            }
        }
        true
    }
}

impl std::ops::Index<usize> for Matrix {
    type Output = Row;
    fn index(&self, row: usize) -> &Self::Output {
        &self.data[row]
    }
}

impl std::ops::IndexMut<usize> for Matrix {
    fn index_mut(&mut self, row: usize) -> &mut Self::Output {
        &mut self.data[row]
    }
}

impl Matrix {
    fn new() -> Matrix {
        Matrix {
            size: 4,
            data: [Row::new([0.0f64; 4]); 4],
        }
    }

    fn new_size(num_rows: usize) -> Matrix {
        // We need additional matrix sizes to calculate the determinant.
        // For now a 2x2 or 3x3 matrix allocates and takes up the same memory as
        // a 4x4. At some point may want to create specialized Matrix3x3 and
        // Matrix2x2 implementations.
        Matrix {
            size: num_rows,
            data: [Row::new([0.0f64; 4]); 4],
        }
    }

    pub fn identity_4x4() -> Matrix {
        let mut matrix = Matrix::new();
        matrix.data[0] = Row::new([1., 0., 0., 0.]);
        matrix.data[1] = Row::new([0., 1., 0., 0.]);
        matrix.data[2] = Row::new([0., 0., 1., 0.]);
        matrix.data[3] = Row::new([0., 0., 0., 1.]);

        matrix
    }

    pub fn multiply_4x4(&self, rhs: &Matrix) -> Matrix {
        if self.size != 4 || rhs.size != 4 {
            panic!("Currently only supports multiplying 4x4 matrices");
        }

        let mut matrix = Matrix::new();
        for row in 0..4 {
            for col in 0..4 {
                matrix.data[row][col] = self.data[row][0] * rhs.data[0][col]
                    + self.data[row][1] * rhs.data[1][col]
                    + self.data[row][2] * rhs.data[2][col]
                    + self.data[row][3] * rhs.data[3][col];
            }
        }
        matrix
    }

    pub fn multiply_point(&self, rhs: &Vector3) -> Vector3 {
        // We only want translation matrices to affect "points" and not "vectors".
        // By setting w to be 1 the point * transform = transformed point in space;
        // If w = 0 then point * transform = only rotated point.
        if self.size != 4 {
            panic!("Currently only supports multiplying 4x4 matrices");
        }

        self.multiply_vector4(&rhs, 1.)
    }

    pub fn multiply_vector(&self, rhs: &Vector3) -> Vector3 {
        // We only want translation matrices to affect "points" and not "vectors".
        // By setting w to be 1 the point * transform = transformed point in space;
        // If w = 0 then point * transform = only rotated point.
        if self.size != 4 {
            panic!("Currently only supports multiplying 4x4 matrices");
        }

        self.multiply_vector4(&rhs, 0.)
    }

    fn multiply_vector4(&self, rhs: &Vector3, w: f64) -> Vector3 {
        // if self.num_rows != 4 || self.num_cols != 4 {
        //     panic!("Currently only supports multiplying 4x4 matrices");
        // }

        let x = self.data[0][0] * rhs.x
            + self.data[0][1] * rhs.y
            + self.data[0][2] * rhs.z
            + self.data[0][3] * w;

        let y = self.data[1][0] * rhs.x
            + self.data[1][1] * rhs.y
            + self.data[1][2] * rhs.z
            + self.data[1][3] * w;

        let z = self.data[2][0] * rhs.x
            + self.data[2][1] * rhs.y
            + self.data[2][2] * rhs.z
            + self.data[2][3] * w;

        // let w = self.data[3][0] * rhs.x
        //     + self.data[3][1] * rhs.y
        //     + self.data[3][2] * rhs.z
        //     + self.data[3][3] * w;

        Vector3::new(x, y, z)
    }

    pub fn transpose(&self) -> Matrix {
        debug_assert!(self.size == 4);
        let mut matrix = Matrix::new();
        for row in 0..self.size {
            for col in 0..self.size {
                matrix.data[row][col] = self.data[col][row];
            }
        }

        matrix
    }

    // Return a copy of a matrix with a given row and column removed
    pub fn submatrix(&self, remove_row: usize, remove_col: usize) -> Matrix {
        debug_assert!(self.size == 4 || self.size == 3);
        let mut matrix = match self.size {
            4 => Matrix::new_size(3),
            3 => Matrix::new_size(2),
            x => panic!(format!("Unexpected matrix size: {}", x)),
        };

        for row in 0..matrix.size {
            let mut actual_row = row;
            if actual_row >= remove_row {
                actual_row += 1;
            }
            for col in 0..matrix.size {
                let mut actual_col = col;
                if actual_col >= remove_col {
                    actual_col += 1;
                }
                matrix.data[row][col] = self.data[actual_row][actual_col];
            }
        }

        matrix
    }

    // The minor of an element at row i and column j is the determinate of the submatrix at (i,j)
    pub fn minor(&self, row: usize, col: usize) -> f64 {
        let sub = self.submatrix(row, col);
        sub.determinant()
    }

    pub fn cofactor(&self, row: usize, col: usize) -> f64 {
        let minor = self.minor(row, col);
        let is_odd = (row + col) % 2 == 1;
        if is_odd {
            return -minor;
        }
        minor
    }

    pub fn determinant(&self) -> f64 {
        if self.size == 2 {
            return self.data[0][0] * self.data[1][1] - self.data[0][1] * self.data[1][0];
        }

        let mut det = 0.0;
        for col in 0..self.size {
            det = det + self.data[0][col] * self.cofactor(0, col);
        }

        det
    }

    pub fn is_invertible(&self) -> bool {
        mathf::approximately(self.determinant(), 0.0) == false
    }

    pub fn inverse(&self) -> Matrix {
        debug_assert!(self.size == 4);
        if !self.is_invertible() {
            panic!("To inverse a matrix it must be invertible");
        }

        let mut matrix = Matrix::new();
        for row in 0..self.size {
            for col in 0..self.size {
                let c = self.cofactor(row, col);

                // note the "[col][row]" here which achieves a transpose
                matrix.data[col][row] = c / self.determinant();
            }
        }

        matrix
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mathf::approximately;

    #[test]
    fn it_creates_a_4x4_matrix() {
        let mut matrix = Matrix::new();
        matrix.data[0][0] = 1.0;
        matrix.data[0][1] = 2.0;
        matrix.data[0][2] = 3.0;
        matrix.data[0][3] = 4.0;

        matrix.data[1][0] = 5.5;
        matrix.data[1][1] = 6.5;
        matrix.data[1][2] = 7.5;
        matrix.data[1][3] = 8.5;

        matrix.data[2][0] = 9.0;
        matrix.data[2][1] = 10.0;
        matrix.data[2][2] = 11.0;
        matrix.data[2][3] = 12.0;

        matrix.data[3][0] = 13.5;
        matrix.data[3][1] = 14.5;
        matrix.data[3][2] = 15.5;
        matrix.data[3][3] = 16.5;

        assert!(approximately(matrix.data[0][0], 1.0));
        assert!(approximately(matrix.data[0][3], 4.0));
        assert!(approximately(matrix.data[1][0], 5.5));
        assert!(approximately(matrix.data[1][2], 7.5));
        assert!(approximately(matrix.data[2][2], 11.0));
        assert!(approximately(matrix.data[3][0], 13.5));
        assert!(approximately(matrix.data[3][2], 15.5));
    }

    #[test]
    fn it_creates_a_2x2_matrix() {
        let mut matrix = Matrix::new_size(2);
        matrix.data[0][0] = -3.0;
        matrix.data[0][1] = 5.0;
        matrix.data[1][0] = 1.0;
        matrix.data[1][1] = -2.0;

        assert!(approximately(matrix.data[0][0], -3.0));
        assert!(approximately(matrix.data[0][1], 5.0));
        assert!(approximately(matrix.data[1][0], 1.0));
        assert!(approximately(matrix.data[1][1], -2.0));
    }

    #[test]
    fn it_creates_a_3x3_matrix() {
        let mut matrix = Matrix::new_size(3);
        matrix.data[0][0] = -3.0;
        matrix.data[0][1] = 5.0;
        matrix.data[0][2] = 0.0;

        matrix.data[1][0] = 1.0;
        matrix.data[1][1] = -2.0;
        matrix.data[1][2] = -7.0;

        matrix.data[2][0] = 0.0;
        matrix.data[2][1] = 1.0;
        matrix.data[2][2] = 1.0;

        assert!(approximately(matrix.data[0][0], -3.0));
        assert!(approximately(matrix.data[1][1], -2.0));
        assert!(approximately(matrix.data[2][2], 1.0));
    }

    #[test]
    fn test_identical_matrices_are_equal() {
        let mut matrix1 = Matrix::new_size(2);
        matrix1.data[0][0] = -3.0;
        matrix1.data[0][1] = 5.0;
        matrix1.data[1][0] = 1.0;
        matrix1.data[1][1] = -2.0;

        let mut matrix2 = Matrix::new_size(2);
        matrix2.data[0][0] = -3.0;
        matrix2.data[0][1] = 5.0;
        matrix2.data[1][0] = 1.0;
        matrix2.data[1][1] = -2.0;

        assert_eq!(matrix1, matrix2);
    }

    #[test]
    fn test_different_matrices_are_not_equal() {
        let mut matrix1 = Matrix::new_size(2);
        matrix1.data[0][0] = -3.0;
        matrix1.data[0][1] = 5.0;
        matrix1.data[1][0] = 1.0;
        matrix1.data[1][1] = -2.0;

        let mut matrix2 = Matrix::new_size(2);
        matrix2.data[0][0] = 1.0;
        matrix2.data[0][1] = 2.0;
        matrix2.data[1][0] = 3.0;
        matrix2.data[1][1] = 4.0;

        assert_ne!(matrix1, matrix2);
    }

    #[test]
    fn test_matrix_multipy_4x4() {
        let mut matrix1 = Matrix::new();
        matrix1.data[0][0] = 1.0;
        matrix1.data[0][1] = 2.0;
        matrix1.data[0][2] = 3.0;
        matrix1.data[0][3] = 4.0;

        matrix1.data[1][0] = 5.0;
        matrix1.data[1][1] = 6.0;
        matrix1.data[1][2] = 7.0;
        matrix1.data[1][3] = 8.0;

        matrix1.data[2][0] = 9.0;
        matrix1.data[2][1] = 8.0;
        matrix1.data[2][2] = 7.0;
        matrix1.data[2][3] = 6.0;

        matrix1.data[3][0] = 5.0;
        matrix1.data[3][1] = 4.0;
        matrix1.data[3][2] = 3.0;
        matrix1.data[3][3] = 2.0;

        let mut matrix2 = Matrix::new();
        matrix2.data[0][0] = -2.0;
        matrix2.data[0][1] = 1.0;
        matrix2.data[0][2] = 2.0;
        matrix2.data[0][3] = 3.0;

        matrix2.data[1][0] = 3.0;
        matrix2.data[1][1] = 2.0;
        matrix2.data[1][2] = 1.0;
        matrix2.data[1][3] = -1.0;

        matrix2.data[2][0] = 4.0;
        matrix2.data[2][1] = 3.0;
        matrix2.data[2][2] = 6.0;
        matrix2.data[2][3] = 5.0;

        matrix2.data[3][0] = 1.0;
        matrix2.data[3][1] = 2.0;
        matrix2.data[3][2] = 7.0;
        matrix2.data[3][3] = 8.0;

        let result = matrix1.multiply_4x4(&matrix2);

        assert!(approximately(result.data[0][0], 20.0));
        assert!(approximately(result.data[0][1], 22.0));
        assert!(approximately(result.data[0][2], 50.0));
        assert!(approximately(result.data[0][3], 48.0));

        assert!(approximately(result.data[1][0], 44.0));
        assert!(approximately(result.data[1][1], 54.0));
        assert!(approximately(result.data[1][2], 114.0));
        assert!(approximately(result.data[1][3], 108.0));

        assert!(approximately(result.data[2][0], 40.0));
        assert!(approximately(result.data[2][1], 58.0));
        assert!(approximately(result.data[2][2], 110.0));
        assert!(approximately(result.data[2][3], 102.0));

        assert!(approximately(result.data[3][0], 16.0));
        assert!(approximately(result.data[3][1], 26.0));
        assert!(approximately(result.data[3][2], 46.0));
        assert!(approximately(result.data[3][3], 42.0));
    }

    #[test]
    fn test_matrix_multiply_vector4() {
        let mut matrix1 = Matrix::new();
        matrix1.data[0][0] = 1.0;
        matrix1.data[0][1] = 2.0;
        matrix1.data[0][2] = 3.0;
        matrix1.data[0][3] = 4.0;

        matrix1.data[1][0] = 2.0;
        matrix1.data[1][1] = 4.0;
        matrix1.data[1][2] = 4.0;
        matrix1.data[1][3] = 2.0;

        matrix1.data[2][0] = 8.0;
        matrix1.data[2][1] = 6.0;
        matrix1.data[2][2] = 4.0;
        matrix1.data[2][3] = 1.0;

        matrix1.data[3][0] = 0.0;
        matrix1.data[3][1] = 0.0;
        matrix1.data[3][2] = 0.0;
        matrix1.data[3][3] = 1.0;

        // let vector = Vector4::new(1.0, 2.0, 3.0, 1.0);
        let vector = Vector3::new(1.0, 2.0, 3.0);

        let result = matrix1.multiply_vector4(&vector, 1.0);

        assert!(approximately(result.x, 18.0));
        assert!(approximately(result.y, 24.0));
        assert!(approximately(result.z, 33.0));
        // assert!(approximately(result.w, 1.0));
    }

    #[test]
    fn test_matrix_multiply_by_identity() {
        let mut matrix1 = Matrix::new();
        matrix1.data[0][0] = 0.0;
        matrix1.data[0][1] = 1.0;
        matrix1.data[0][2] = 2.0;
        matrix1.data[0][3] = 4.0;

        matrix1.data[1][0] = 1.0;
        matrix1.data[1][1] = 2.0;
        matrix1.data[1][2] = 4.0;
        matrix1.data[1][3] = 8.0;

        matrix1.data[2][0] = 2.0;
        matrix1.data[2][1] = 4.0;
        matrix1.data[2][2] = 8.0;
        matrix1.data[2][3] = 16.0;

        matrix1.data[3][0] = 4.0;
        matrix1.data[3][1] = 8.0;
        matrix1.data[3][2] = 16.0;
        matrix1.data[3][3] = 32.0;

        let result = matrix1.multiply_4x4(&Matrix::identity_4x4());
        assert_eq!(matrix1, result);
    }

    #[test]
    fn test_multiply_identity_4x4_by_vector4() {
        let matrix = Matrix::identity_4x4();
        // let vector = Vector4::new(1.0, 2.0, 3.0, 4.0);
        // let result = matrix.multiply_vector4(&vector);
        let vector = Vector3::new(1.0, 2.0, 3.0);
        let result = matrix.multiply_vector4(&vector, 0.);

        assert_eq!(vector.x, result.x);
        assert_eq!(vector.y, result.y);
        assert_eq!(vector.z, result.z);
        // assert_eq!(vector.w, result.w);
    }

    #[test]
    fn test_matrix_transpose() {
        let mut matrix1 = Matrix::new();
        matrix1.data[0][0] = 0.0;
        matrix1.data[0][1] = 9.0;
        matrix1.data[0][2] = 3.0;
        matrix1.data[0][3] = 0.0;

        matrix1.data[1][0] = 9.0;
        matrix1.data[1][1] = 8.0;
        matrix1.data[1][2] = 0.0;
        matrix1.data[1][3] = 8.0;

        matrix1.data[2][0] = 1.0;
        matrix1.data[2][1] = 8.0;
        matrix1.data[2][2] = 5.0;
        matrix1.data[2][3] = 3.0;

        matrix1.data[3][0] = 0.0;
        matrix1.data[3][1] = 0.0;
        matrix1.data[3][2] = 5.0;
        matrix1.data[3][3] = 8.0;

        let mut expected = Matrix::new();
        expected.data[0][0] = 0.0;
        expected.data[0][1] = 9.0;
        expected.data[0][2] = 1.0;
        expected.data[0][3] = 0.0;

        expected.data[1][0] = 9.0;
        expected.data[1][1] = 8.0;
        expected.data[1][2] = 8.0;
        expected.data[1][3] = 0.0;

        expected.data[2][0] = 3.0;
        expected.data[2][1] = 0.0;
        expected.data[2][2] = 5.0;
        expected.data[2][3] = 5.0;

        expected.data[3][0] = 0.0;
        expected.data[3][1] = 8.0;
        expected.data[3][2] = 3.0;
        expected.data[3][3] = 8.0;

        let result = matrix1.transpose();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_matrix_transpose_identity_matrix() {
        let identity_matrix = Matrix::identity_4x4();
        let result = identity_matrix.transpose();
        assert_eq!(result, identity_matrix);
    }

    #[test]
    fn test_matrix_submatrix_3x3() {
        let mut matrix = Matrix::new_size(3);
        matrix.data[0][0] = 1.0;
        matrix.data[0][1] = 5.0;
        matrix.data[0][2] = 0.0;

        matrix.data[1][0] = -3.0;
        matrix.data[1][1] = 2.0;
        matrix.data[1][2] = 7.0;

        matrix.data[2][0] = 0.0;
        matrix.data[2][1] = 6.0;
        matrix.data[2][2] = -3.0;

        let mut expected = Matrix::new_size(2);
        expected.data[0][0] = -3.0;
        expected.data[0][1] = 2.0;
        expected.data[1][0] = 0.0;
        expected.data[1][1] = 6.0;

        let submatrix = matrix.submatrix(0, 2);
        assert_eq!(submatrix.size, 2);
        assert_eq!(submatrix, expected);
    }

    #[test]
    fn test_matrix_submatrix_4x4() {
        let mut matrix = Matrix::new();
        matrix.data[0][0] = -6.0;
        matrix.data[0][1] = 1.0;
        matrix.data[0][2] = 1.0;
        matrix.data[0][3] = 6.0;

        matrix.data[1][0] = -8.0;
        matrix.data[1][1] = 5.0;
        matrix.data[1][2] = 8.0;
        matrix.data[1][3] = 6.0;

        matrix.data[2][0] = -1.0;
        matrix.data[2][1] = 0.0;
        matrix.data[2][2] = 8.0;
        matrix.data[2][3] = 2.0;

        matrix.data[3][0] = -7.0;
        matrix.data[3][1] = 1.0;
        matrix.data[3][2] = -1.0;
        matrix.data[3][3] = 1.0;

        let mut expected = Matrix::new_size(3);
        expected.data[0][0] = -6.0;
        expected.data[0][1] = 1.0;
        expected.data[0][2] = 6.0;

        expected.data[1][0] = -8.0;
        expected.data[1][1] = 8.0;
        expected.data[1][2] = 6.0;

        expected.data[2][0] = -7.0;
        expected.data[2][1] = -1.0;
        expected.data[2][2] = 1.0;

        let submatrix = matrix.submatrix(2, 1);
        assert_eq!(submatrix.size, 3);
        assert_eq!(submatrix, expected);
    }

    #[test]
    fn test_matrix_minor_3x3() {
        let mut matrix = Matrix::new_size(3);
        matrix.data[0][0] = 3.0;
        matrix.data[0][1] = 5.0;
        matrix.data[0][2] = 0.0;

        matrix.data[1][0] = 2.0;
        matrix.data[1][1] = -1.0;
        matrix.data[1][2] = -7.0;

        matrix.data[2][0] = 6.0;
        matrix.data[2][1] = -1.0;
        matrix.data[2][2] = 5.0;

        assert_eq!(matrix.minor(1, 0), 25.0);

        let matrix_b = matrix.submatrix(1, 0);
        assert_eq!(matrix_b.determinant(), 25.0);
    }

    #[test]
    fn test_3x3_matrix_cofactor() {
        let mut matrix = Matrix::new_size(3);
        matrix.data[0][0] = 3.0;
        matrix.data[0][1] = 5.0;
        matrix.data[0][2] = 0.0;

        matrix.data[1][0] = 2.0;
        matrix.data[1][1] = -1.0;
        matrix.data[1][2] = -7.0;

        matrix.data[2][0] = 6.0;
        matrix.data[2][1] = -1.0;
        matrix.data[2][2] = 5.0;

        assert_eq!(matrix.minor(0, 0), -12.0);
        assert_eq!(matrix.cofactor(0, 0), -12.0);
        assert_eq!(matrix.minor(1, 0), 25.0);
        assert_eq!(matrix.cofactor(1, 0), -25.0);
    }

    #[test]
    fn test_2x2_matrix_determinant() {
        let mut matrix = Matrix::new_size(2);
        matrix.data[0][0] = 1.0;
        matrix.data[0][1] = 5.0;
        matrix.data[1][0] = -3.0;
        matrix.data[1][1] = 2.0;

        let determinate = matrix.determinant();
        assert!(approximately(determinate, 17.0));
    }

    #[test]
    fn test_3x3_matrix_determinate() {
        let mut matrix = Matrix::new_size(3);
        matrix.data[0][0] = 1.0;
        matrix.data[0][1] = 2.0;
        matrix.data[0][2] = 6.0;

        matrix.data[1][0] = -5.0;
        matrix.data[1][1] = 8.0;
        matrix.data[1][2] = -4.0;

        matrix.data[2][0] = 2.0;
        matrix.data[2][1] = 6.0;
        matrix.data[2][2] = 4.0;

        assert_eq!(matrix.cofactor(0, 0), 56.0);
        assert_eq!(matrix.cofactor(0, 1), 12.0);
        assert_eq!(matrix.cofactor(0, 2), -46.0);
        assert_eq!(matrix.determinant(), -196.0);
    }

    #[test]
    fn test_4x4_matrix_determinate() {
        let mut matrix = Matrix::new();
        matrix.data[0][0] = -2.0;
        matrix.data[0][1] = -8.0;
        matrix.data[0][2] = 3.0;
        matrix.data[0][3] = 5.0;

        matrix.data[1][0] = -3.0;
        matrix.data[1][1] = 1.0;
        matrix.data[1][2] = 7.0;
        matrix.data[1][3] = 3.0;

        matrix.data[2][0] = 1.0;
        matrix.data[2][1] = 2.0;
        matrix.data[2][2] = -9.0;
        matrix.data[2][3] = 6.0;

        matrix.data[3][0] = -6.0;
        matrix.data[3][1] = 7.0;
        matrix.data[3][2] = 7.0;
        matrix.data[3][3] = -9.0;

        assert_eq!(matrix.cofactor(0, 0), 690.0);
        assert_eq!(matrix.cofactor(0, 1), 447.0);
        assert_eq!(matrix.cofactor(0, 2), 210.0);
        assert_eq!(matrix.cofactor(0, 3), 51.0);
        assert_eq!(matrix.determinant(), -4071.0);
    }

    #[test]
    fn test_4x4_matrix_is_invertible() {
        let mut matrix = Matrix::new();
        matrix.data[0][0] = 6.0;
        matrix.data[0][1] = 4.0;
        matrix.data[0][2] = 4.0;
        matrix.data[0][3] = 4.0;

        matrix.data[1][0] = 5.0;
        matrix.data[1][1] = 5.0;
        matrix.data[1][2] = 7.0;
        matrix.data[1][3] = 6.0;

        matrix.data[2][0] = 4.0;
        matrix.data[2][1] = -9.0;
        matrix.data[2][2] = 3.0;
        matrix.data[2][3] = -7.0;

        matrix.data[3][0] = 9.0;
        matrix.data[3][1] = 1.0;
        matrix.data[3][2] = 7.0;
        matrix.data[3][3] = -6.0;

        assert!(matrix.is_invertible());
    }

    #[test]
    fn test_4x4_matrix_is_not_invertible() {
        let mut matrix = Matrix::new();
        matrix.data[0][0] = -4.0;
        matrix.data[0][1] = 2.0;
        matrix.data[0][2] = -2.0;
        matrix.data[0][3] = -3.0;

        matrix.data[1][0] = 9.0;
        matrix.data[1][1] = 6.0;
        matrix.data[1][2] = 2.0;
        matrix.data[1][3] = 6.0;

        matrix.data[2][0] = 0.0;
        matrix.data[2][1] = -5.0;
        matrix.data[2][2] = 1.0;
        matrix.data[2][3] = -5.0;

        matrix.data[3][0] = 0.0;
        matrix.data[3][1] = 0.0;
        matrix.data[3][2] = 0.0;
        matrix.data[3][3] = 0.0;

        assert!(!matrix.is_invertible());
    }

    #[test]
    fn test_4x4_matrix_inverse() {
        let mut matrix = Matrix::new();
        matrix.data[0][0] = -5.0;
        matrix.data[0][1] = 2.0;
        matrix.data[0][2] = 6.0;
        matrix.data[0][3] = -8.0;

        matrix.data[1][0] = 1.0;
        matrix.data[1][1] = -5.0;
        matrix.data[1][2] = 1.0;
        matrix.data[1][3] = 8.0;

        matrix.data[2][0] = 7.0;
        matrix.data[2][1] = 7.0;
        matrix.data[2][2] = -6.0;
        matrix.data[2][3] = -7.0;

        matrix.data[3][0] = 1.0;
        matrix.data[3][1] = -3.0;
        matrix.data[3][2] = 7.0;
        matrix.data[3][3] = 4.0;

        let inverted_matrix = matrix.inverse();
        assert_eq!(matrix.determinant(), 532.0);
        assert_eq!(matrix.cofactor(2, 3), -160.0);
        assert_eq!(inverted_matrix.data[3][2], -160.0 / 532.0);
        assert_eq!(matrix.cofactor(3, 2), 105.0);
        assert_eq!(inverted_matrix.data[2][3], 105.0 / 532.0);

        let mut expected = Matrix::new();
        expected.data[0][0] = 0.21805;
        expected.data[0][1] = 0.45113;
        expected.data[0][2] = 0.24060;
        expected.data[0][3] = -0.04511;

        expected.data[1][0] = -0.80827;
        expected.data[1][1] = -1.45677;
        expected.data[1][2] = -0.44361;
        expected.data[1][3] = 0.52068;

        expected.data[2][0] = -0.07895;
        expected.data[2][1] = -0.22368;
        expected.data[2][2] = -0.05263;
        expected.data[2][3] = 0.19737;

        expected.data[3][0] = -0.52256;
        expected.data[3][1] = -0.81391;
        expected.data[3][2] = -0.30075;
        expected.data[3][3] = 0.30639;
        assert_eq!(inverted_matrix, expected);
    }

    #[test]
    fn test_4x4_matrix_inverse_2() {
        let mut matrix = Matrix::new();
        matrix.data[0][0] = 8.0;
        matrix.data[0][1] = -5.0;
        matrix.data[0][2] = 9.0;
        matrix.data[0][3] = 2.0;

        matrix.data[1][0] = 7.0;
        matrix.data[1][1] = 5.0;
        matrix.data[1][2] = 6.0;
        matrix.data[1][3] = 1.0;

        matrix.data[2][0] = -6.0;
        matrix.data[2][1] = 0.0;
        matrix.data[2][2] = 9.0;
        matrix.data[2][3] = 6.0;

        matrix.data[3][0] = -3.0;
        matrix.data[3][1] = 0.0;
        matrix.data[3][2] = -9.0;
        matrix.data[3][3] = -4.0;

        let inverted_matrix = matrix.inverse();

        let mut expected = Matrix::new();
        expected.data[0][0] = -0.15385;
        expected.data[0][1] = -0.15385;
        expected.data[0][2] = -0.28205;
        expected.data[0][3] = -0.53846;

        expected.data[1][0] = -0.07692;
        expected.data[1][1] = 0.12308;
        expected.data[1][2] = 0.02564;
        expected.data[1][3] = 0.03077;

        expected.data[2][0] = 0.35897;
        expected.data[2][1] = 0.35897;
        expected.data[2][2] = 0.43590;
        expected.data[2][3] = 0.92308;

        expected.data[3][0] = -0.69231;
        expected.data[3][1] = -0.69231;
        expected.data[3][2] = -0.76923;
        expected.data[3][3] = -1.92308;
        assert_eq!(inverted_matrix, expected);
    }

    #[test]
    fn test_4x4_matrix_inverse_3() {
        let mut matrix = Matrix::new();
        matrix.data[0][0] = 9.0;
        matrix.data[0][1] = 3.0;
        matrix.data[0][2] = 0.0;
        matrix.data[0][3] = 9.0;

        matrix.data[1][0] = -5.0;
        matrix.data[1][1] = -2.0;
        matrix.data[1][2] = -6.0;
        matrix.data[1][3] = -3.0;

        matrix.data[2][0] = -4.0;
        matrix.data[2][1] = 9.0;
        matrix.data[2][2] = 6.0;
        matrix.data[2][3] = 4.0;

        matrix.data[3][0] = -7.0;
        matrix.data[3][1] = 6.0;
        matrix.data[3][2] = 6.0;
        matrix.data[3][3] = 2.0;

        let inverted_matrix = matrix.inverse();

        let mut expected = Matrix::new();
        expected.data[0][0] = -0.04074;
        expected.data[0][1] = -0.07778;
        expected.data[0][2] = 0.14444;
        expected.data[0][3] = -0.22222;

        expected.data[1][0] = -0.07778;
        expected.data[1][1] = 0.03333;
        expected.data[1][2] = 0.36667;
        expected.data[1][3] = -0.33333;

        expected.data[2][0] = -0.02901;
        expected.data[2][1] = -0.14630;
        expected.data[2][2] = -0.10926;
        expected.data[2][3] = 0.12963;

        expected.data[3][0] = 0.17778;
        expected.data[3][1] = 0.06667;
        expected.data[3][2] = -0.26667;
        expected.data[3][3] = 0.33333;
        assert_eq!(inverted_matrix, expected);
    }

    #[test]
    fn test_4x4_matrix_inverse_inverse() {
        // Testing that if you multiply a matrix A by another matrix B, producing C,
        // you can multiply C by the inverse of B to get A again.
        let mut matrix_a = Matrix::new();
        matrix_a.data[0][0] = 3.0;
        matrix_a.data[0][1] = -9.0;
        matrix_a.data[0][2] = 7.0;
        matrix_a.data[0][3] = 3.0;

        matrix_a.data[1][0] = 8.0;
        matrix_a.data[1][1] = -8.0;
        matrix_a.data[1][2] = 2.0;
        matrix_a.data[1][3] = -9.0;

        matrix_a.data[2][0] = -4.0;
        matrix_a.data[2][1] = 4.0;
        matrix_a.data[2][2] = 4.0;
        matrix_a.data[2][3] = 1.0;

        matrix_a.data[3][0] = -6.0;
        matrix_a.data[3][1] = 5.0;
        matrix_a.data[3][2] = -1.0;
        matrix_a.data[3][3] = 1.0;

        let mut matrix_b = Matrix::new();
        matrix_b.data[0][0] = 8.0;
        matrix_b.data[0][1] = 2.0;
        matrix_b.data[0][2] = 2.0;
        matrix_b.data[0][3] = 2.0;

        matrix_b.data[1][0] = 3.0;
        matrix_b.data[1][1] = -1.0;
        matrix_b.data[1][2] = 7.0;
        matrix_b.data[1][3] = -0.0;

        matrix_b.data[2][0] = 7.0;
        matrix_b.data[2][1] = 0.0;
        matrix_b.data[2][2] = 5.0;
        matrix_b.data[2][3] = 4.0;

        matrix_b.data[3][0] = 6.0;
        matrix_b.data[3][1] = -2.0;
        matrix_b.data[3][2] = 0.0;
        matrix_b.data[3][3] = 5.0;

        let matrix_c = matrix_a.multiply_4x4(&matrix_b);

        let result = matrix_c.multiply_4x4(&matrix_b.inverse());
        assert_eq!(result, matrix_a);
    }
}
