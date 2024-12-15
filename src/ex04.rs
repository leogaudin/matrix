#[path ="./structs.rs"]
mod structs;
use structs::Vector;

pub fn test_04() {
    let u = Vector::from(vec![0., 0., 0.]);
    println!("{}, {}, {}\n", u.norm_1(), u.norm(), u.norm_inf());
    // 0.0, 0.0, 0.0
    let u = Vector::from(vec![1., 2., 3.]);
    println!("{}, {}, {}\n", u.norm_1(), u.norm(), u.norm_inf());
    // 6.0, 3.74165738, 3.0
    let u = Vector::from(vec![-1., -2.]);
    println!("{}, {}, {}\n", u.norm_1(), u.norm(), u.norm_inf());
    // 3.0, 2.236067977, 2.0
}
