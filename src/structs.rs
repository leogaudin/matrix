use std::{fmt, ops};
pub trait Operations<K> {
    fn shape(&self) -> (usize, usize);
    fn is_square(&self) -> bool;
    fn reshape(&mut self, shape: (usize, usize));
    fn add(&mut self, v: &Self);
    fn sub(&mut self, v: &Self);
    fn scl(&mut self, a: K);
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
    > Operations<K> for Matrix<K>
{
    fn shape(&self) -> (usize, usize) {
        return self.shape;
    }

    fn is_square(&self) -> bool {
        return self.shape.0 == self.shape.1;
    }

    fn reshape(&mut self, shape: (usize, usize)) {
        if shape.0 * shape.1 != self.shape.0 * self.shape.1 {
            panic!(
                "Array of size {} cannot be reshaped to size {}",
                self.shape.0 * self.shape.1,
                shape.0 * shape.1
            );
        }
        self.shape = shape;
    }

    fn add(&mut self, v: &Matrix<K>) {
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

    fn sub(&mut self, v: &Matrix<K>) {
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

    fn scl(&mut self, a: K) {
        for i in 0..self.data.len() {
            self.data[i] = self.data[i] * a;
        }
    }
}

// print! and println!
impl<K: std::fmt::Display> fmt::Display for Matrix<K> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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

    fn add(self, v: Self) -> Self {
        let new_matrix = self.matrix + v.matrix;
        return Vector { matrix: new_matrix };
    }
}

// Subtraction overload
impl<K: std::fmt::Display + ops::Sub<Output = K> + Copy> ops::Sub for Vector<K> {
    type Output = Self;

    fn sub(self, v: Self) -> Self {
        let new_matrix = self.matrix - v.matrix;
        return Vector { matrix: new_matrix };
    }
}

// Scalar multiplication overload
impl<K: std::fmt::Display + ops::Mul<Output = K> + Copy> ops::Mul<K> for Vector<K> {
    type Output = Self;

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
    > Operations<K> for Vector<K>
{
    fn shape(&self) -> (usize, usize) {
        return self.matrix.shape();
    }

    fn is_square(&self) -> bool {
        return self.matrix.is_square();
    }

    fn reshape(&mut self, shape: (usize, usize)) {
        self.matrix.reshape(shape);
    }

    fn add(&mut self, v: &Vector<K>) {
        self.matrix.add(&v.matrix);
    }

    fn sub(&mut self, v: &Vector<K>) {
        self.matrix.sub(&v.matrix);
    }

    fn scl(&mut self, a: K) {
        self.matrix.scl(a);
    }
}

// print! and println!
impl<K: std::fmt::Display> fmt::Display for Vector<K> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.matrix)
    }
}
