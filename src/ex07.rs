#[path = "./structs.rs"]
mod structs;
use structs::{Vector, Matrix};

pub fn test_07() {
    let u: Matrix<f64> = Matrix::from(vec![vec![1., 0.], vec![0., 1.]]);
    let v = Vector::from(vec![4., 2.]);
    println!("{}\n", u.mul_vec(v));
    // [4.]
    // [2.]
    let u = Matrix::from(vec![vec![2., 0.], vec![0., 2.]]);
    let v = Vector::from(vec![4., 2.]);
    println!("{}\n", u.mul_vec(v));
    // [8.]
    // [4.]
    let u = Matrix::from(vec![vec![2., -2.], vec![-2., 2.]]);
    let v = Vector::from(vec![4., 2.]);
    println!("{}\n", u.mul_vec(v));
    // [4.]
    // [-4.]
    let u = Matrix::from(vec![vec![1., 0.], vec![0., 1.]]);
    let v = Matrix::from(vec![vec![1., 0.], vec![0., 1.]]);
    println!("{}\n", u.mul_mat(v));
    // [1., 0.]
    // [0., 1.]
    let u = Matrix::from(vec![vec![1., 0.], vec![0., 1.]]);
    let v = Matrix::from(vec![vec![2., 1.], vec![4., 2.]]);
    println!("{}\n", u.mul_mat(v));
    // [2., 1.]
    // [4., 2.]
    let u = Matrix::from(vec![vec![3., -5.], vec![6., 8.]]);
    let v = Matrix::from(vec![vec![2., 1.], vec![4., 2.]]);
    println!("{}\n", u.mul_mat(v));
    // [-14., -7.]
    // [44., 22.]
    let u = Matrix::from(vec![
        vec![3., 1., 1., 4.],
        vec![5., 3., 2., 1.],
        vec![6., 2., 9., 5.],
    ]);
    let v = Matrix::from(vec![
        vec![4., 9.],
        vec![6., 8.],
        vec![9., 7.],
        vec![7., 6.],
    ]);
    println!("{}\n", u.mul_mat(v));
    // [55, 66]
    // [63, 89]
    // [152, 163]
}
