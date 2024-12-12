pub trait Operations<K> {
    fn shape(&self) -> (usize, usize);
    fn is_square(&self) -> bool;
    fn print(&self);
    fn reshape(&mut self, shape: (usize, usize));
    fn add(&mut self, v: &Self);
    fn sub(&mut self, v: &Self);
    fn scl(&mut self, a: K);
}

// MATRIX

pub struct Matrix<K: std::fmt::Display> {
    shape: (usize, usize),
    data: Vec<K>,
}

impl<K: std::fmt::Display + Copy> Matrix<K> {
    pub fn new(data: Vec<Vec<K>>) -> Self {
        let shape: (usize, usize) = (data[0].len(), data.len());
        let mut flat: Vec<K> = Vec::new();
        for i in 0..shape.0 {
            for j in 0..shape.1 {
                flat.push(data[j][i]);
            }
        }
        return Matrix {
            shape,
            data: flat,
        };
    }
}

impl<K: std::fmt::Display + std::ops::Add<Output = K> + std::ops::Sub<Output = K> + std::ops::Mul<Output = K> + Copy>
Operations<K> for Matrix<K> {
    fn shape(&self) -> (usize, usize) {
        return self.shape;
    }

    fn is_square(&self) -> bool {
        return self.shape.0 == self.shape.1;
    }

    fn print(&self) {
        for i in 0..self.shape.0 {
            print!("[");
            for j in 0..self.shape.1 {
                print!("{}", self.data[j * self.shape.0 + i]);
                if j < self.shape.1 - 1 {
                    print!(", ");
                }
            }
            print!("]");
            println!();
        }
        println!();
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


// VECTOR

pub struct Vector<K: std::fmt::Display> {
    matrix: Matrix<K>,
}

impl<K: std::fmt::Display + Copy> Vector<K> {
    pub fn new(data: Vec<K>) -> Self {
        let matrix = Matrix::new(vec![data]);
        return Vector {
            matrix,
        };
    }
}

impl<K: std::fmt::Display + std::ops::Add<Output = K> + std::ops::Sub<Output = K> + std::ops::Mul<Output = K> + Copy>
Operations<K> for Vector<K> {
    fn shape(&self) -> (usize, usize) {
        return self.matrix.shape();
    }

    fn is_square(&self) -> bool {
        return self.matrix.is_square();
    }

    fn print(&self) {
        self.matrix.print();
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
