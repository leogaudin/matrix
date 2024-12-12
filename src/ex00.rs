#[path = "./structs.rs"]
mod structs;
use structs::{Matrix, Vector, Operations};
use colored::*;

pub fn test_00() {
	println!("\nTESTING {}", "EX00".bold());
    let mut u = Vector::new(vec![2., 3.]);
    let v = Vector::new(vec![5., 7.]);
    u.add(&v);
    u.print();
    // [7.0]
    // [10.0]

    let mut u = Vector::new(vec![2., 3.]);
    let v = Vector::new(vec![5., 7.]);
    u.sub(&v);
    u.print();
    // [-3.0]
    // [-4.0]

    let mut u = Vector::new(vec![2., 3.]);
    u.scl(2.);
    u.print();
    // [4.0]
    // [6.0]

    let mut u = Matrix::new(vec![vec![1., 2.], vec![3., 4.]]);
    let v = Matrix::new(vec![vec![7., 4.], vec![-2., 2.]]);
    u.add(&v);
    u.print();
    // [8.0, 6.0]
    // [1.0, 6.0]

    let mut u = Matrix::new(vec![vec![1., 2.], vec![3., 4.]]);
    let v = Matrix::new(vec![vec![7., 4.], vec![-2., 2.]]);
    u.sub(&v);
    u.print();
    // [-6.0, -2.0]
    // [5.0, 2.0]

    let mut u = Matrix::new(vec![vec![1., 2.], vec![3., 4.]]);
    u.scl(2.);
    u.print();
    // [2.0, 4.0]
    // [6.0, 8.0]
}
