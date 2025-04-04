use crate::core::Vector;
use crate::operations::Operations;

pub fn linear_combination<
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
    v: &[Vector<K>],
    a: &[K],
) -> Vector<K> {
    if v.len() != a.len() {
        panic!("");
    }

    let mut accum = v[0].clone();
    accum.scl(a[0]);
    for i in 1..v.len() {
        let mut current = v[i].clone();
        current.scl(a[i]);
        accum.add(&current);
    }

    return accum;
}

pub fn test_01() {
    let e1 = Vector::from(vec![1., 0., 0.]);
    let e2 = Vector::from(vec![0., 1., 0.]);
    let e3 = Vector::from(vec![0., 0., 1.]);
    let v1 = Vector::from(vec![1., 2., 3.]);
    let v2 = Vector::from(vec![0., 10., -100.]);
    println!("{}\n", linear_combination(&[e1, e2, e3], &[10., -2., 0.5]));
    // [10.]
    // [-2.]
    // [0.5]
    println!("{}\n", linear_combination(&[v1, v2], &[10., -2.]));
    // [10.]
    // [0.]
    // [230.]
}
