use crate::core::{Vector, Complex};

pub fn test_03() {
    let u = Vector::from(vec![0., 0.]);
    let v = Vector::from(vec![1., 1.]);
    println!("{}\n", u.dot(v));
    // 0.0
    let u = Vector::from(vec![1., 1.]);
    let v = Vector::from(vec![1., 1.]);
    println!("{}\n", u.dot(v));
    // 2.0
    let u = Vector::from(vec![-1., 6.]);
    let v = Vector::from(vec![3., 2.]);
    println!("{}\n", u.dot(v));
    // 9.0

    let u = Vector::from(vec![Complex::from((1., 2.)), Complex::from((3., 4.))]);
    let v = Vector::from(vec![Complex::from((5., 6.)), Complex::from((7., 8.))]);
    println!("{}\n", u.dot(v));
}
