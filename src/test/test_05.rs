use crate::core::Vector;
use crate::operations::Operations;

pub fn angle_cos<
    K: std::fmt::Display
        + std::ops::Add<Output = K>
        + std::ops::Sub<Output = K>
        + std::ops::Mul<Output = K>
        + std::ops::Div<Output = K>
        + std::cmp::PartialEq
        + std::cmp::PartialOrd
        + std::ops::Neg<Output = K>
        + Default
        + Copy
        + Operations
>(
    u: &Vector<K>,
    v: &Vector<K>,
) -> K {
    return u.dot(v.clone()) / (u.norm() * v.norm());
}

pub fn test_05() {
    let u = Vector::from(vec![1., 0.]);
    let v = Vector::from(vec![1., 0.]);
    println!("{}\n", angle_cos(&u, &v));
    // 1.0
    let u = Vector::from(vec![1., 0.]);
    let v = Vector::from(vec![0., 1.]);
    println!("{}\n", angle_cos(&u, &v));
    // 0.0
    let u = Vector::from(vec![-1., 1.]);
    let v = Vector::from(vec![1., -1.]);
    println!("{}\n", angle_cos(&u, &v));
    // -1.0
    let u = Vector::from(vec![2., 1.]);
    let v = Vector::from(vec![4., 2.]);
    println!("{}\n", angle_cos(&u, &v));
    // 1.0
    let u = Vector::from(vec![1., 2., 3.]);
    let v = Vector::from(vec![4., 5., 6.]);
    println!("{}\n", angle_cos(&u, &v));
    // 0.974631846
}
