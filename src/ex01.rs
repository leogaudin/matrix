#[path = "./structs.rs"]
mod structs;
use structs::{Matrix, Operations, Vector};

pub fn linear_combination<
K: std::fmt::Display
    + std::ops::Add<Output = K>
    + std::ops::Sub<Output = K>
    + std::ops::Mul<Output = K>
    + Copy,
>(
    v: &[Vector<K>],
    a: &[K],
) -> Vector<K> {
    if v.len() != a.len() {
        panic!("");
    }
    let mut scaled = v.iter().map(|x| x.clone()).collect::<Vec<Vector<K>>>();
    for i in 0..a.len() {
        scaled[i].scl(a[i]);
    }

    let mut accum = Vector::from(scaled[0].clone());
    for i in 1..scaled.len() {
        accum.add(&scaled[i]);
    }

    return accum;
}

pub fn test_01() {
    let e1 = Vector::from_1d(vec![1., 0., 0.]);
    let e2 = Vector::from_1d(vec![0., 1., 0.]);
    let e3 = Vector::from_1d(vec![0., 0., 1.]);
    let v1 = Vector::from_1d(vec![1., 2., 3.]);
    let v2 = Vector::from_1d(vec![0., 10., -100.]);
    println!("{}\n", linear_combination(&[e1, e2, e3], &[10., -2., 0.5]));
    // [10.]
    // [-2.]
    // [0.5]
    println!("{}\n", linear_combination(&[v1, v2], &[10., -2.]));
    // [10.]
    // [0.]
    // [230.]
}
