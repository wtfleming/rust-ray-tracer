use crate::mathf;
use crate::mathf::vector4;
use crate::mathf::vector4::Vector4;

#[derive(Debug)]
pub struct Matrix {
    pub num_rows: usize,
    pub num_cols: usize,
    pub data: Vec<std::vec::Vec<f64>>,
}

pub fn new(num_rows: usize, num_cols: usize) -> Matrix {
    Matrix {
        num_rows,
        num_cols,
        data: vec![vec![0.0f64; num_rows]; num_cols],
    }
}

pub fn identity_4x4() -> Matrix {
    let mut matrix = Matrix {
        num_rows: 4,
        num_cols: 4,
        data: vec![vec![0.0f64; 4]; 4],
    };

    matrix.data[0][0] = 1.0;
    matrix.data[1][1] = 1.0;
    matrix.data[2][2] = 1.0;
    matrix.data[3][3] = 1.0;

    matrix
}

impl Matrix {
    pub fn equals(&self, rhs: &Matrix) -> bool {
        for r in 0..self.num_rows {
            for c in 0..self.num_cols {
                if !mathf::approximately(self.data[r][c], rhs.data[r][c]) {
                    return false;
                }
            }
        }
        true
    }

    pub fn multiply_4x4(&self, rhs: &Matrix) -> Matrix {
        if self.num_rows != 4 || self.num_cols != 4 || rhs.num_rows != 4 || rhs.num_cols != 4 {
            panic!("Currently only supports multiplying 4x4 matrices");
        }

        let mut matrix = new(4, 4);
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

    pub fn multiply_vector4(&self, rhs: &Vector4) -> Vector4 {
        if self.num_rows != 4 || self.num_cols != 4 {
            panic!("Currently only supports multiplying 4x4 matrices");
        }

        let mut vector = vector4::new(0.0, 0.0, 0.0, 0.0);

        vector.x = self.data[0][0] * rhs.x
            + self.data[0][1] * rhs.y
            + self.data[0][2] * rhs.z
            + self.data[0][3] * rhs.w;

        vector.y = self.data[1][0] * rhs.x
            + self.data[1][1] * rhs.y
            + self.data[1][2] * rhs.z
            + self.data[1][3] * rhs.w;

        vector.z = self.data[2][0] * rhs.x
            + self.data[2][1] * rhs.y
            + self.data[2][2] * rhs.z
            + self.data[2][3] * rhs.w;

        vector.w = self.data[3][0] * rhs.x
            + self.data[3][1] * rhs.y
            + self.data[3][2] * rhs.z
            + self.data[3][3] * rhs.w;

        vector
    }

    pub fn transpose(&self) -> Matrix {
        let mut matrix = new(self.num_rows, self.num_cols);
        for row in 0..self.num_rows {
            for col in 0..self.num_cols {
                matrix.data[row][col] = self.data[col][row];
            }
        }

        matrix
    }

    // Return a copy of a matrix with a given row and column removed
    pub fn submatrix(&self, remove_row: usize, remove_col: usize) -> Matrix {
        let mut matrix = new(self.num_rows - 1, self.num_cols - 1);

        for row in 0..matrix.num_rows {
            let mut actual_row = row;
            if actual_row >= remove_row {
                actual_row += 1;
            }
            for col in 0..matrix.num_cols {
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
        if self.num_rows == 2 || self.num_cols == 2 {
            return self.data[0][0] * self.data[1][1] - self.data[0][1] * self.data[1][0];
        }

        let mut det = 0.0;
        for col in 0..self.num_cols {
            det = det + self.data[0][col] * self.cofactor(0, col);
        }

        det
    }
}

#[cfg(test)]
mod tests {
    use super::super::approximately;
    use super::*;

    #[test]
    fn it_creates_a_4x4_matrix() {
        let mut matrix = new(4, 4);
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
        let mut matrix = new(2, 2);
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
        let mut matrix = new(3, 3);
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
        let mut matrix1 = new(2, 2);
        matrix1.data[0][0] = -3.0;
        matrix1.data[0][1] = 5.0;
        matrix1.data[1][0] = 1.0;
        matrix1.data[1][1] = -2.0;

        let mut matrix2 = new(2, 2);
        matrix2.data[0][0] = -3.0;
        matrix2.data[0][1] = 5.0;
        matrix2.data[1][0] = 1.0;
        matrix2.data[1][1] = -2.0;

        assert!(matrix1.equals(&matrix2));
    }

    #[test]
    fn test_different_matrices_are_not_equal() {
        let mut matrix1 = new(2, 2);
        matrix1.data[0][0] = -3.0;
        matrix1.data[0][1] = 5.0;
        matrix1.data[1][0] = 1.0;
        matrix1.data[1][1] = -2.0;

        let mut matrix2 = new(2, 2);
        matrix2.data[0][0] = 1.0;
        matrix2.data[0][1] = 2.0;
        matrix2.data[1][0] = 3.0;
        matrix2.data[1][1] = 4.0;

        assert!(!matrix1.equals(&matrix2));
    }

    #[test]
    fn test_matrix_multipy_4x4() {
        let mut matrix1 = new(4, 4);
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

        let mut matrix2 = new(4, 4);
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
        let mut matrix1 = new(4, 4);
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

        let vector = vector4::new(1.0, 2.0, 3.0, 1.0);

        let result = matrix1.multiply_vector4(&vector);

        assert!(approximately(result.x, 18.0));
        assert!(approximately(result.y, 24.0));
        assert!(approximately(result.z, 33.0));
        assert!(approximately(result.w, 1.0));
    }

    #[test]
    fn test_matrix_multiply_by_identity() {
        let mut matrix1 = new(4, 4);
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

        let result = matrix1.multiply_4x4(&identity_4x4());
        assert!(matrix1.equals(&result));
    }

    #[test]
    fn test_multiply_identity_4x4_by_vector4() {
        let matrix = identity_4x4();
        let vector = vector4::new(1.0, 2.0, 3.0, 4.0);
        let result = matrix.multiply_vector4(&vector);

        assert_eq!(vector.x, result.x);
        assert_eq!(vector.y, result.y);
        assert_eq!(vector.z, result.z);
        assert_eq!(vector.w, result.w);
    }

    #[test]
    fn test_matrix_transpose() {
        let mut matrix1 = new(4, 4);
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

        let mut expected = new(4, 4);
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
        assert!(result.equals(&expected));
    }

    #[test]
    fn test_matrix_transpose_identity_matrix() {
        let identity_matrix = identity_4x4();
        let result = identity_matrix.transpose();
        assert!(result.equals(&identity_matrix));
    }

    #[test]
    fn test_matrix_submatrix_3x3() {
        let mut matrix = new(3, 3);
        matrix.data[0][0] = 1.0;
        matrix.data[0][1] = 5.0;
        matrix.data[0][2] = 0.0;

        matrix.data[1][0] = -3.0;
        matrix.data[1][1] = 2.0;
        matrix.data[1][2] = 7.0;

        matrix.data[2][0] = 0.0;
        matrix.data[2][1] = 6.0;
        matrix.data[2][2] = -3.0;

        let mut expected = new(2, 2);
        expected.data[0][0] = -3.0;
        expected.data[0][1] = 2.0;
        expected.data[1][0] = 0.0;
        expected.data[1][1] = 6.0;

        let submatrix = matrix.submatrix(0, 2);
        assert_eq!(submatrix.num_rows, 2);
        assert_eq!(submatrix.num_cols, 2);
        assert!(submatrix.equals(&expected));
    }

    #[test]
    fn test_matrix_submatrix_4x4() {
        let mut matrix = new(4, 4);
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

        let mut expected = new(3, 3);
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
        assert_eq!(submatrix.num_rows, 3);
        assert_eq!(submatrix.num_cols, 3);
        assert!(submatrix.equals(&expected));
    }

    #[test]
    fn test_matrix_minor_3x3() {
        let mut matrix = new(3, 3);
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
        let mut matrix = new(3, 3);
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
        let mut matrix = new(2, 2);
        matrix.data[0][0] = 1.0;
        matrix.data[0][1] = 5.0;
        matrix.data[1][0] = -3.0;
        matrix.data[1][1] = 2.0;

        let determinate = matrix.determinant();
        assert!(approximately(determinate, 17.0));
    }

    #[test]
    fn test_3x3_matrix_determinate() {
        let mut matrix = new(3, 3);
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
        let mut matrix = new(4, 4);
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
}
