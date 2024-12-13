#[path = "./structs.rs"]
mod structs;
use structs::{Matrix, Vector, Operations};
use colored::*;

pub fn test_00() {
	println!("\nTESTING {}", "EX00".bold());
    let mut u = Vector::from_1d(vec![2., 3.]);
    let v = Vector::from_1d(vec![5., 7.]);
    u.add(&v);
    println!("{}\n", u);
    // [7.0]
    // [10.0]

    let mut u = Vector::from_1d(vec![2., 3.]);
    let v = Vector::from_1d(vec![5., 7.]);
    u.sub(&v);
    println!("{}\n", u);
    // [-3.0]
    // [-4.0]

    let mut u = Vector::from_1d(vec![2., 3.]);
    u.scl(2.);
    println!("{}\n", u);
    // [4.0]
    // [6.0]

    let mut u = Matrix::from_2d(vec![vec![1., 2.], vec![3., 4.]]);
    let v = Matrix::from_2d(vec![vec![7., 4.], vec![-2., 2.]]);
    u.add(&v);
    println!("{}\n", u);
    // [8.0, 6.0]
    // [1.0, 6.0]

    let mut u = Matrix::from_2d(vec![vec![1., 2.], vec![3., 4.]]);
    let v = Matrix::from_2d(vec![vec![7., 4.], vec![-2., 2.]]);
    u.sub(&v);
    println!("{}\n", u);
    // [-6.0, -2.0]
    // [5.0, 2.0]

    let mut u = Matrix::from_2d(vec![vec![1., 2.], vec![3., 4.], vec![5., 6.]]);
    u.scl(2.);
    println!("{}\n", u);
    // [2.0, 4.0]
    // [6.0, 8.0]
    // [10.0, 12.0]
}
