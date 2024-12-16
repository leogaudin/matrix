use std::{fmt, ops};

pub fn to_f32<K: std::fmt::Display>(a: K) -> f32 {
    return a.to_string().parse::<f32>().unwrap();
}

// MATRIX

// Struct
pub struct Matrix<K: std::fmt::Display> {
    shape: (usize, usize),
    data: Vec<K>,
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
        for i in 0..data.1 .0 {
            for j in 0..data.1 .1 {
                row_major.push(data.0[j * data.1 .0 + i]);
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

    pub fn reshape(&mut self, shape: (usize, usize)) {
        if shape.0 * shape.1 != self.shape.0 * self.shape.1 {
            panic!(
                "Array of size {} cannot be reshaped to size {}",
                self.shape.0 * self.shape.1,
                shape.0 * shape.1
            );
        }
        self.shape = shape;
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

        for j in 0..self.shape.1 {
            for i in 0..self.shape.0 {
                result.push(self.data[i * self.shape.1 + j]);
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

        for j in 0..self.shape.1 {
            let mut column: Vec<K> = Vec::new();
            for i in 0..self.shape.0 {
                column.push(self.data[i * self.shape.1 + j]);
            }
            let mut vector: Vector<K> = Vector::from(column);
            vector.scl(vec.flat()[j]);

            if j == 0 {
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

        for j in 0..mat_shape.1 {
            let mut column: Vec<K> = Vec::new();
            for i in 0..mat_shape.0 {
                column.push(mat_flat[i * mat_shape.1 + j]);
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

        let mut sum = self.data[0];
        for i in 1..self.shape.0 {
            sum = sum + self.data[i * self.shape.0 + i];
        }
        return sum;
    }
}

// print! and println!
impl<K: std::fmt::Display> fmt::Display for Matrix<K> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in 0..self.shape.0 {
            write!(f, "[")?;
            for j in 0..self.shape.1 {
                write!(f, "{}", self.data[i * self.shape.1 + j])?;
                if j < self.shape.1 - 1 {
                    write!(f, ", ")?;
                }
            }
            write!(f, "]")?;
            if i < self.shape.0 - 1 {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

// VECTOR

// Struct
pub struct Vector<K: std::fmt::Display> {
    matrix: Matrix<K>,
}

// From 1D
impl<K: std::fmt::Display + Copy> From<Vec<K>> for Vector<K> {
    fn from(data: Vec<K>) -> Self {
        return Vector {
            matrix: Matrix::from((data.clone(), (data.len(), 1))),
        };
    }
}

// Clone
impl<K: std::fmt::Display + Copy> Clone for Vector<K> {
    fn clone(&self) -> Self {
        return Vector {
            matrix: self.matrix.clone(),
        };
    }
}

// Addition overload
impl<K: std::fmt::Display + ops::Add<Output = K> + Copy> ops::Add for Vector<K> {
    type Output = Self;

    // Time: O(n) − Space: O(n)
    // where n is the length of the vector
    fn add(self, v: Self) -> Self {
        let new_matrix = self.matrix + v.matrix;
        return Vector { matrix: new_matrix };
    }
}

// Subtraction overload
impl<K: std::fmt::Display + ops::Sub<Output = K> + Copy> ops::Sub for Vector<K> {
    type Output = Self;

    // Time: O(n) − Space: O(n)
    // where n is the length of the vector
    fn sub(self, v: Self) -> Self {
        let new_matrix = self.matrix - v.matrix;
        return Vector { matrix: new_matrix };
    }
}

// Scalar multiplication overload
impl<K: std::fmt::Display + ops::Mul<Output = K> + Copy> ops::Mul<K> for Vector<K> {
    type Output = Self;

    // Time: O(n) − Space: O(n)
    // where n is the length of the vector
    fn mul(self, a: K) -> Self {
        let new_matrix = self.matrix * a;
        return Vector { matrix: new_matrix };
    }
}

// Operations
impl<
        K: std::fmt::Display
            + std::ops::Add<Output = K>
            + std::ops::Sub<Output = K>
            + std::ops::Mul<Output = K>
            + Copy,
    > Vector<K>
{
    pub fn shape(&self) -> (usize, usize) {
        return self.matrix.shape();
    }

    pub fn flat(&self) -> Vec<K> {
        return self.matrix.flat();
    }

    pub fn is_square(&self) -> bool {
        return self.matrix.is_square();
    }

    pub fn reshape(&mut self, shape: (usize, usize)) {
        self.matrix.reshape(shape);
    }

    // Time: O(n) − Space: O(1)
    // where n is the length of the vector
    pub fn add(&mut self, v: &Vector<K>) {
        self.matrix.add(&v.matrix);
    }

    // Time: O(n) − Space: O(1)
    pub fn sub(&mut self, v: &Vector<K>) {
        self.matrix.sub(&v.matrix);
    }

    // Time: O(n) − Space: O(1)
    pub fn scl(&mut self, a: K) {
        self.matrix.scl(a);
    }

    // Time: O(n) − Space: O(1)
    pub fn dot(&self, v: Vector<K>) -> f32 {
        if self.shape() != v.shape() || self.flat().len() != v.flat().len() {
            panic!(
                "Shapes {:?} and {:?} are incompatible",
                self.shape(),
                v.shape()
            );
        }
        let u = self.flat();
        let v = v.flat();

        let mut sum: f32 = to_f32(u[0] * v[0]);
        for i in 1..self.flat().len() {
            sum = sum + to_f32(u[i] * v[i]);
        }

        return sum;
    }

    // Time: O(n) − Space: O(1)
    pub fn norm_1(&self) -> f32 {
        // Manhattan
        let mut sum = to_f32(self.matrix.data[0]).abs();
        for i in 1..self.flat().len() {
            sum = sum + to_f32(self.matrix.data[i]).abs();
        }
        return sum;
    }

    // Time: O(n) − Space: O(1)
    pub fn norm(&self) -> f32 {
        // Euclidian
        let mut squared_sum = to_f32(self.matrix.data[0]).powi(2);
        for i in 1..self.flat().len() {
            squared_sum = squared_sum + to_f32(self.matrix.data[i]).powi(2);
        }
        return squared_sum.powf(0.5);
    }

    // Time: O(n) − Space: O(1)
    pub fn norm_inf(&self) -> f32 {
        // Supremum
        let mut max = to_f32(self.matrix.data[0]).abs();
        for i in 1..self.flat().len() {
            let abs = to_f32(self.matrix.data[i]).abs();
            if abs > max {
                max = abs;
            }
        }
        return max;
    }
}

// print! and println!
impl<K: std::fmt::Display> fmt::Display for Vector<K> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.matrix)
    }
}
