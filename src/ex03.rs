#[path = "./structs.rs"]
mod structs;
use structs::Vector;

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
}
