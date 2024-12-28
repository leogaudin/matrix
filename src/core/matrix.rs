use std::{fmt, ops};
use crate::core::vector::Vector;

// Struct
pub struct Matrix<K: std::fmt::Display> {
    shape: (usize, usize),
    pub data: Vec<K>,
}

// From 2D
impl<K: std::fmt::Display + Copy> From<Vec<Vec<K>>> for Matrix<K> {
    fn from(data: Vec<Vec<K>>) -> Self {
        if data.len() * data[0].len() != data.iter().flatten().count() {
            panic!(
                "Array of size {} cannot be reshaped to size {}",
                data.iter().flatten().count(),
                data.len() * data[0].len()
            );
        }
        return Matrix {
            shape: (data.len(), data[0].len()),
            data: data.iter().flatten().copied().collect(),
        };
    }
}

// From 1D
impl<K: std::fmt::Display + Copy> From<(Vec<K>, (usize, usize))> for Matrix<K> {
    fn from(data: (Vec<K>, (usize, usize))) -> Self {
        // Reads the data in column-major order
        if data.1 .0 * data.1 .1 != data.0.len() {
            panic!(
                "Array of size {} cannot be reshaped to size {}",
                data.0.len(),
                data.1 .0 * data.1 .1
            );
        }

        let mut row_major = Vec::new();
        for r in 0..data.1 .0 {
            for c in 0..data.1 .1 {
                row_major.push(data.0[c * data.1 .0 + r]);
            }
        }

        return Matrix {
            shape: data.1,
            data: row_major,
        };
    }
}

// Clone
impl<K: std::fmt::Display + Copy> Clone for Matrix<K> {
    fn clone(&self) -> Self {
        return Matrix {
            shape: self.shape,
            data: self.data.clone(),
        };
    }
}

// Addition overload
impl<K: std::fmt::Display + ops::Add<Output = K> + Copy> ops::Add for Matrix<K> {
    type Output = Self;

    // Time: O(n) − Space: O(n)
    // where n is the number of elements in the matrix
    fn add(self, v: Self) -> Self {
        if self.shape != v.shape {
            panic!("Shapes {:?} and {:?} are incompatible", self.shape, v.shape);
        }

        let mut new_data = Vec::new();
        for i in 0..self.data.len() {
            new_data.push(self.data[i] + v.data[i]);
        }

        return Matrix {
            shape: self.shape,
            data: new_data,
        };
    }
}

// Subtraction overload
impl<K: std::fmt::Display + ops::Sub<Output = K> + Copy> ops::Sub for Matrix<K> {
    type Output = Self;

    // Time: O(n) − Space: O(n)
    // where n is the number of elements in the matrix
    fn sub(self, v: Self) -> Self {
        if self.shape != v.shape {
            panic!("Shapes {:?} and {:?} are incompatible", self.shape, v.shape);
        }

        let mut new_data = Vec::new();
        for i in 0..self.data.len() {
            new_data.push(self.data[i] - v.data[i]);
        }

        return Matrix {
            shape: self.shape,
            data: new_data,
        };
    }
}

// Scalar multiplication overload
impl<K: std::fmt::Display + ops::Mul<Output = K> + Copy> ops::Mul<K> for Matrix<K> {
    type Output = Self;

    // Time: O(n) − Space: O(n)
    // where n is the number of elements in the matrix
    fn mul(self, a: K) -> Self {
        let mut new_data = Vec::new();
        for i in 0..self.data.len() {
            new_data.push(self.data[i] * a);
        }

        return Matrix {
            shape: self.shape,
            data: new_data,
        };
    }
}

// Operations
impl<
        K: std::fmt::Display
            + std::ops::Add<Output = K>
            + std::ops::Sub<Output = K>
            + std::ops::Mul<Output = K>
            + std::ops::Div<Output = K>
            + std::cmp::PartialEq
            + std::ops::Neg<Output = K>
            + Default
            + Copy,
    > Matrix<K>
{
    pub fn shape(&self) -> (usize, usize) {
        return self.shape;
    }

    pub fn flat(&self) -> Vec<K> {
        return self.data.clone();
    }

    pub fn is_square(&self) -> bool {
        return self.shape.0 == self.shape.1;
    }

    // pub fn reshape(&mut self, shape: (usize, usize)) {
    //     if shape.0 * shape.1 != self.shape.0 * self.shape.1 {
    //         panic!(
    //             "Array of size {} cannot be reshaped to size {}",
    //             self.shape.0 * self.shape.1,
    //             shape.0 * shape.1
    //         );
    //     }
    //     self.shape = shape;
    // }

    pub fn get(&self, r: usize, c: usize) -> K {
        return self.data[r * self.shape.1 + c];
    }

    pub fn set(&mut self, r: usize, c: usize, value: K) {
        self.data[r * self.shape.1 + c] = value;
    }

    // Time: O(n) − Space: O(1)
    // where n is the number of elements in the matrix
    pub fn add(&mut self, v: &Matrix<K>) {
        if self.shape != v.shape() {
            panic!(
                "Shapes {:?} and {:?} are incompatible",
                self.shape,
                v.shape()
            );
        }

        for i in 0..self.data.len() {
            self.data[i] = self.data[i] + v.data[i];
        }
    }

    // Time: O(n) − Space: O(1)
    // where n is the number of elements in the matrix
    pub fn sub(&mut self, v: &Matrix<K>) {
        if self.shape != v.shape() {
            panic!(
                "Shapes {:?} and {:?} are incompatible",
                self.shape,
                v.shape()
            );
        }

        for i in 0..self.data.len() {
            self.data[i] = self.data[i] - v.data[i];
        }
    }

    // Time: O(n) − Space: O(1)
    // where n is the number of elements in the matrix
    pub fn scl(&mut self, a: K) {
        for i in 0..self.data.len() {
            self.data[i] = self.data[i] * a;
        }
    }

    // Time: O(nm) − Space: O(nm)
    // where self is a matrix of shape (m, n)
    pub fn transpose(&mut self) {
        let mut result: Vec<K> = Vec::new();

        for c in 0..self.shape.1 {
            for r in 0..self.shape.0 {
                result.push(self.get(r, c));
            }
        }

        self.data = result;
        self.shape = (self.shape.1, self.shape.0)
    }

    // Time: O(nm) − Space: O(n)
    // where self is a matrix of shape (m, n)
    // and vec is a vector of shape (n, 1)
    pub fn mul_vec(&self, vec: Vector<K>) -> Vector<K> {
        if self.shape.1 != vec.shape().0 {
            panic!(
                "The vector shape {:?} is incompatible with the matrix shape {:?}\
                Expected a vector of shape ({}, 1)",
                self.shape,
                vec.shape(),
                self.shape.1,
            );
        }

        let mut result: Vector<K> = Vector::from(vec![self.data[0]; self.shape.0]);

        for c in 0..self.shape.1 {
            let mut column: Vec<K> = Vec::new();
            for r in 0..self.shape.0 {
                column.push(self.get(r, c));
            }
            let mut vector: Vector<K> = Vector::from(column);
            vector.scl(vec.flat()[c]);

            if c == 0 {
                result = vector;
            } else {
                result.add(&vector);
            }
        }

        return Vector::from(result.clone());
    }

    // Time: O(nmp) − Space: O(np)
    // where self is a matrix of shape (m, n)
    // and mat is a matrix of shape (n, p)
    pub fn mul_mat(&self, mat: Matrix<K>) -> Matrix<K> {
        let mat_shape: (usize, usize) = mat.shape();
        let mat_flat: Vec<K> = mat.flat();
        if self.shape.1 != mat_shape.0 {
            panic!(
                "The vector shape {:?} is incompatible with the matrix shape {:?}\
                Expected a vector of shape ({}, 1)",
                self.shape,
                mat_shape,
                self.shape.1,
            );
        }

        let mut result: Vec<Vec<K>> = Vec::new(); // p elements of length n

        for c in 0..mat_shape.1 {
            let mut column: Vec<K> = Vec::new();
            for r in 0..mat_shape.0 {
                column.push(mat_flat[r * mat_shape.1 + c]);
            }
            // p mul_vecs:
            // Each one has time complexity nm → mul_mat: O(nmp)
            // Each one has space complexity n → mul_mat: O(np)
            let vector: Vector<K> = self.mul_vec(Vector::from(column));

            result.push(vector.flat());
        }

        let mut matrix: Matrix<K> = Matrix::from(result);
        matrix.transpose();
        return matrix;
    }

    // Time: O(n) − Space: 0(1)
    pub fn trace(&self) -> K {
        if !self.is_square() {
            panic!("The matrix is not square");
        }

        let mut sum = K::default();
        for r in 0..self.shape.0 {
            sum = sum + self.get(r, r);
        }
        return sum;
    }

    // Time: O(m^2 * n) − Space: O(1)
    // where self is a matrix of shape (m, n)
    pub fn row_echelon(&self) -> Matrix<K> {
        let mut matrix: Matrix<K> = self.clone();
        let (rows, columns) = matrix.shape();
        let mut pvt_column: usize = 0; // Pivot column

        'out: for curr in 0..rows {
            // If pvt reaches max
            if pvt_column == columns {
                break;
            }

            let mut nz = curr;

            // Find the row with a non-zero element
            // in the current pvt_column
            while matrix.get(nz, pvt_column) == K::default() {
                nz += 1;

                // If no non-zero element is found,
                // move to the next column
                if nz == rows {
                    nz = curr;
                    pvt_column += 1;

                    // If the current pvt_column reached
                    // the number of columns, break the loop
                    if columns == pvt_column {
                        break 'out;
                    }
                }
            }

            // Swap the current row with the first row
            // containing the non-zero element
            for c in 0..columns {
                let temp = matrix.get(curr, c);
                matrix.set(curr, c, matrix.get(nz, c));
                matrix.set(nz, c, temp);
            }

            let divisor: K = matrix.get(curr, pvt_column);
            if divisor != K::default() {
                for c in 0..columns {
                    matrix.set(curr, c, matrix.get(curr, c) / divisor);
                }
            }

            // For each row except the current one,
            // subtract the product of the cell in the pivot column
            // and the corresponding cell in the current row.
            // (i.e., the cell below the pivot cell is zeroed)
            for r in curr..rows {
                if r != curr {
                    let first = matrix.get(r, pvt_column);
                    for c in 0..columns {
                        matrix.set(
                            r, c,
                            matrix.get(r, c) - (first * matrix.get(curr, c))
                        );
                    }
                }
            }

            // Make the numbers above the pivot cell zero
            for r in 0..curr {
                let first = matrix.get(r, pvt_column);
                for c in 0..columns {
                    matrix.set(
                        r, c,
                        matrix.get(r, c) - (first * matrix.get(curr, c))
                    );
                }
            }

            // Move to the next column
            pvt_column += 1;
        }

        return matrix;
    }

    // Time: O(n^3) − Space: O(n^2)
    pub fn determinant(&self) -> K {
        if !self.is_square() {
            panic!(
                "Shape {:?} is not a square",
                self.shape
            );
        }

        if self.shape.0 == 2 {
            return self.data[0] * self.data[3] - self.data[1] * self.data[2];
        }

        let mut negative: bool = false;
        let mut sum: K = K::default();

        for col in 0..self.shape.0 {
            let mut sub_matrix: Vec<Vec<K>> = Vec::new();
            for r in 1..self.shape.0 {
                let mut row: Vec<K> = Vec::new();
                for c in 0..self.shape.1 {
                    if c != col {
                        row.push(self.get(r, c));
                    }
                }
                sub_matrix.push(row);
            }

            let sub_matrix: Matrix<K> = Matrix::from(sub_matrix);
            let sub_determinant: K = sub_matrix.determinant();
            if negative {
                sum = sum - self.get(0, col) * sub_determinant;
            } else {
                sum = sum + self.get(0, col) * sub_determinant;
            }
            negative = !negative;
        }

        return sum;
    }

    // Time: O(n^3) − Space: O(n^2)
    // (would be space O(1) but we need to check if the matrix is singular)
    pub fn inverse(&self) -> Result<Matrix<K>, &'static str> {
        if !self.is_square() {
            panic!(
                "Shape {:?} is not a square",
                self.shape
            );
        }

        let determinant = self.determinant();

        if determinant == K::default() {
            return Err("The matrix is singular");
        }

        let mut augmented: Vec<Vec<K>> = Vec::new();
        for _ in 0..self.shape.0 {
            let mut row: Vec<K> = Vec::new();
            for _ in 0..self.shape.1 * 2 {
                row.push(K::default());
            }
            augmented.push(row);
        }

        for r in 0..self.shape.0 {
            for c in 0..self.shape.1 {
                augmented[r][c] = self.get(r, c);
            }
            augmented[r][self.shape.1 + r] = determinant / determinant;
        }

        let mut augmented: Matrix<K> = Matrix::from(augmented);
        augmented = augmented.row_echelon();

        let mut unaugmented: Vec<Vec<K>> = Vec::new();
        for _ in 0..self.shape.0 {
            let mut row: Vec<K> = Vec::new();
            for _ in 0..self.shape.1 {
                row.push(K::default());
            }
            unaugmented.push(row);
        }

        for r in 0..self.shape.0 {
            for c in 0..self.shape.1 {
                unaugmented[r][c] = augmented.get(r, c + self.shape.1);
            }
        }

        let unaugmented: Matrix<K> = Matrix::from(unaugmented);

        return Ok(unaugmented);
    }

    pub fn rank(&self) -> usize {
        let rref = self.row_echelon();

        let mut rank: usize = 0;
        for r in 0..rref.shape.0 {
            for c in 0..rref.shape.1 {
                if rref.get(r, c) != K::default() {
                    rank += 1;
                    break;
                }
            }
        }

        return rank;
    }
}

// print! and println!
impl<K: std::fmt::Display> fmt::Display for Matrix<K> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for r in 0..self.shape.0 {
            write!(f, "[")?;
            for c in 0..self.shape.1 {
                write!(f, "{}", self.data[r * self.shape.1 + c])?;
                if c < self.shape.1 - 1 {
                    write!(f, ", ")?;
                }
            }
            write!(f, "]")?;
            if r < self.shape.0 - 1 {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}
