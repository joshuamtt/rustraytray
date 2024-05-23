#![allow(dead_code)]
#![allow(unused_mut)]
#![allow(unused_variables)]

pub mod matrix {
    use crate::{float_equal, PorvTuple}; // @ interesting when include in a lib.rs this

    //becomes unable to be referenced
    //use crate::PorvTuple;

    pub static IDENTITY_MATRIX: [[f32; 4]; 4] = [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ];

    #[derive(Clone)]
    pub struct Matrix {
        // Prefer array since when creating a matrix variable it will
        // typically be called matrix of some sort
        // Or may change to vec.
        pub array: Vec<Vec<f32>>,
    }

    impl Matrix {
        pub fn new(matrix_size: usize, init_value: f32) -> Self {
            // Right now only 2, 3, 4 sizes...
            if matrix_size < 1 || matrix_size > 4 {
                panic!("Given incorrect size to matrix initialization.\n");
            }

            Matrix {
                // For now, just initialized to 0!
                array: vec![vec![init_value; matrix_size]; matrix_size],
            }
        }
        // turn rows into columns and columns into rows.
        pub fn transpose(matrix: &Self) -> Self {
            if matrix.array.len() != 4 {
                println!("Warning! Only has been tested with 4x4 matrices.")
            }
            let mut transposed_matrix = Matrix::new(4, 0.0);

            for i in 0..matrix.array.len() {
                for j in 0..matrix.array.len() {
                    transposed_matrix.array[i][j] = matrix.array[j][i];
                }
            }

            transposed_matrix
        }
        /// Returns the identity matrix
        pub fn identity_matrix() -> Matrix {
            Matrix {
                array: {
                    vec![
                        vec![1.0, 0.0, 0.0, 0.0],
                        vec![0.0, 1.0, 0.0, 0.0],
                        vec![0.0, 0.0, 1.0, 0.0],
                        vec![0.0, 0.0, 0.0, 1.0],
                    ]
                },
            }
        }
        /// determinant of a 2x2 matrix.
        /// To be changed once I optimize out the use of VEC.
        pub fn determinant(&self) -> f32 {
            let mut determinant = 0.0;
            if self.array.len() == 2 {
                return self.array[0][0] * self.array[1][1] - self.array[0][1] * self.array[1][0];
            } else {
                for column in 0..self.array.len() {
                    // Clonining a matrix...
                    // @speed, @preciseness?
                    // Basically how should I handle moved values in a loop.

                    determinant =
                        determinant + (self.array[0][column] * Matrix::cofactor(self, 0, column));
                }
            }

            determinant
        }

        pub fn submatrix(matrix: &Self, row: usize, column: usize) -> Matrix {
            // Will return a copy of matrix, with row and column removed.
            let matrix_len = matrix.array.len();

            let mut new_matrix = Matrix::new(matrix_len - 1, 0.0);
            // I want to skip row, and column of matrix. BUT I want to fill in row
            // Need to keep track of two different indices...
            // For example, if row = 0, i want the index of rows for matrix to
            // advance but not the index of rows for new matrix.

            for (i, k) in (0..matrix.array.len())
                .filter(|i| *i != row)
                .zip(0..new_matrix.array.len())
            {
                for (j, l) in (0..matrix.array.len())
                    .filter(|j| *j != column)
                    .zip(0..matrix.array.len())
                {
                    new_matrix.array[k][l] = matrix.array[i][j];
                }
            }

            new_matrix
        }
        pub fn minor(matrix: &Self, row: usize, column: usize) -> f32 {
            let sub_matrix = Matrix::submatrix(&matrix, row, column);

            let minor = sub_matrix.determinant();

            minor
        }
        pub fn cofactor(matrix: &Self, row: usize, column: usize) -> f32 {
            let mut minor = Matrix::minor(matrix, row, column);

            if (row + column) % 2 != 0 {
                minor = -(minor);
            }

            minor // Cofactor.
        }
        // @ CHANGE all params to just Self. since we are in the
        pub fn invertible(matrix: &Self) -> bool {
            let determinant = matrix.determinant();

            if determinant == 0.0 {
                return false;
            }

            true
        }
        // @WC
        // - I am not sure If panicing on the matrix being invertible is
        // - the correct way to go.
        // - Is there a situation where I will need to
        pub fn inverse(matrix: &Self) -> Matrix {
            let mut inverted_matrix = Matrix::new(matrix.array.len(), 0.0);

            let determinant = Matrix::determinant(matrix);
            println!("{}", determinant);

            if !(Matrix::invertible(matrix)) {
                panic!("Not invertible.");
            } else {
                for row in 0..matrix.array.len() {
                    for col in 0..matrix.array.len() {
                        let cofactor = Matrix::cofactor(matrix, row, col);

                        inverted_matrix.array[col][row] = cofactor / determinant;
                    }
                }
            }

            inverted_matrix
        }

        pub fn translation(x: f32, y: f32, z: f32) -> Matrix {
            // We start with an identity matrix.
            let mut translation_matrix = Matrix::identity_matrix();

            // and then just add some values.
            translation_matrix.array[0][3] = x;
            translation_matrix.array[1][3] = y;
            translation_matrix.array[2][3] = z;

            translation_matrix
        }
        /// Changes a vectors length
        pub fn scaling(x: f32, y: f32, z: f32) -> Matrix {
            // We start off with the identity matrix
            let mut transformation_matrix = Matrix::identity_matrix();

            transformation_matrix.array[0][0] = x;
            transformation_matrix.array[1][1] = y;
            transformation_matrix.array[2][2] = z;

            transformation_matrix
        }

        pub fn rotation_x(radians: f32) -> Matrix {
            let mut rotation_matrix = Matrix::identity_matrix();

            rotation_matrix.array[1][1] = radians.cos();
            rotation_matrix.array[2][1] = radians.sin();
            rotation_matrix.array[1][2] = -(radians.sin());
            rotation_matrix.array[2][2] = radians.cos();

            rotation_matrix
        }

        pub fn rotation_y(radians: f32) -> Matrix {
            let mut rotation_matrix = Matrix::identity_matrix();

            rotation_matrix.array[0][0] = radians.cos();
            rotation_matrix.array[0][2] = radians.sin();
            rotation_matrix.array[2][0] = -(radians.sin());
            rotation_matrix.array[2][2] = radians.cos();

            rotation_matrix
        }
        pub fn rotation_z(radians: f32) -> Matrix {
            let mut rotation_matrix = Matrix::identity_matrix();

            rotation_matrix.array[0][0] = radians.cos();
            rotation_matrix.array[0][1] = -(radians.sin());
            rotation_matrix.array[1][0] = radians.sin();
            rotation_matrix.array[1][1] = radians.cos();

            rotation_matrix
        }
        pub fn shearing(x_y: f32, x_z: f32, y_x: f32, y_z: f32, z_x: f32, z_y: f32) -> Matrix {
            let mut shearing_matrix = Matrix::identity_matrix();

            shearing_matrix.array[0][1] = x_y;
            shearing_matrix.array[0][2] = x_z;

            shearing_matrix.array[1][0] = y_x;
            shearing_matrix.array[0][1] = y_z;

            shearing_matrix.array[2][0] = z_x;
            shearing_matrix.array[2][1] = z_y;

            shearing_matrix
        }
    }

    impl std::fmt::Display for Matrix {
        // TODO: Add padding for the digit length
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            for i in 0..self.array.len() {
                for j in 0..self.array.len() {
                    write!(f, "| {} ", self.array[i][j]).unwrap();
                }
                write!(f, "|\n").unwrap();
            }
            Ok(())
        }
    }
    impl PartialEq for Matrix {
        fn eq(&self, other: &Matrix) -> bool {
            if self.array.len() != other.array.len() {
                panic!("Unable to perform equality on these matrices sizes. EQ is implemented for only square matrices.");
            }
            // This will flatten both self and other
            // then compare each element.
            for (elem1, elem2) in self
                .array
                .iter()
                .flatten()
                .zip(other.array.iter().flatten())
            {
                if !(float_equal(*elem1, *elem2)) {
                    return false;
                }
            }

            true
        }
    }

    //// Associative, not commutative
    //// Transformations must be concatenated in reverse order to have them applied in the order that is needed.
    //// -> EX: Rotate, and then scale, you will need to multiple the translation matrix by the scaling matrix, and then by the rotation matrix.

    impl std::ops::Mul for Matrix {
        type Output = Matrix;

        fn mul(self, rhs: Self) -> Self::Output {
            // Only implemented for 4x4 matricies.
            if self.array.len() != 4 && rhs.array.len() != 4 {
                panic!("Matrix Multiplication only implemented for 4x4!");
            }
            let mut out_matrix = Matrix::new(4, 0.0);

            for row in 0..self.array.len() {
                for col in 0..self.array.len() {
                    out_matrix.array[row][col] = (self.array[row][0] * rhs.array[0][col])
                        + (self.array[row][1] * rhs.array[1][col])
                        + (self.array[row][2] * rhs.array[2][col])
                        + (self.array[row][3] * rhs.array[3][col]);
                }
            }

            out_matrix
        }
    }

    impl std::ops::Div<f32> for Matrix {
        type Output = Matrix;

        fn div(self, rhs: f32) -> Self::Output {
            let mut new_matrix = Matrix::new(self.array.len(), 0.0);

            for i in 0..self.array.len() {
                for j in 0..self.array.len() {
                    new_matrix.array[i][j] = self.array[i][j] / rhs;
                }
            }

            new_matrix
        }
    }
    impl std::ops::Mul<PorvTuple> for Matrix {
        type Output = PorvTuple;

        fn mul(self, rhs: PorvTuple) -> Self::Output {
            // So you are supposed to be able to multiply a matrix by a tuple and get a tuple back I don't think it needs to be PorVTuple, though.
            // I will implement this when I know more about what is needed for the program.
            // Looks like it is supposed to be a PorVTuple

            // let ret_tuple = PorvTuple::default(); // P or V section will be overwritten at the end.

            let mut static_arr: [f32; 4] = [0.0; 4];
            for i in 0..self.array.len() {
                static_arr[i] = self.array[i][0] * rhs.x
                    + self.array[i][1] * rhs.y
                    + self.array[i][2] * rhs.z
                    + self.array[i][3] * rhs.w // This will either be 0, or whatever self.array[i][3] is * 1
            }

            PorvTuple {
                x: static_arr[0],
                y: static_arr[1],
                z: static_arr[2],
                w: rhs.w,
            }
        }
    }
}
