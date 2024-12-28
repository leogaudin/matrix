use std::{fmt, ops};
use crate::core::matrix::Matrix;

pub fn to_f32<K: std::fmt::Display>(a: K) -> f32 {
    return a.to_string().parse::<f32>().unwrap();
}

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
            + std::ops::Div<Output = K>
            + std::cmp::PartialEq
            + std::ops::Neg<Output = K>
            + Default
            + Copy,
    > Vector<K>
{
    pub fn shape(&self) -> (usize, usize) {
        return self.matrix.shape();
    }

    pub fn flat(&self) -> Vec<K> {
        return self.matrix.flat();
    }

    // pub fn get(&self, i: usize) -> K {
    //     return self.matrix.get(i, 0);
    // }

    // pub fn set(&mut self, i: usize, value: K) {
    //     self.matrix.set(i, 0, value);
    // }

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

        let mut sum: f32 = f32::default();
        for i in 0..self.flat().len() {
            sum = sum + to_f32(u[i] * v[i]);
        }

        return sum;
    }

    // Time: O(n) − Space: O(1)
    pub fn norm_1(&self) -> f32 {
        // Manhattan
        let mut sum = f32::default();
        for i in 0..self.flat().len() {
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
        let mut max = f32::default();
        for i in 0..self.flat().len() {
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
