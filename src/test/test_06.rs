use crate::core::Vector;
use crate::operations::Operations;

fn cross_product<
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
) -> Vector<K> {
    if u.shape().0 != 3 || v.shape().0 != 3 {
        panic!(
            "Both vectors must have shape (3, 1). Received {:?} and {:?}.",
            u.shape(),
            v.shape()
        );
    }

    let a = u.flat();
    let b = v.flat();

    return Vector::from(vec![
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0]
    ]);
}

pub fn test_06() {
    let u = Vector::from(vec![0., 0., 1.]);
    let v = Vector::from(vec![1., 0., 0.]);
    println!("{}\n", cross_product(&u, &v));
    // [0.]
    // [1.]
    // [0.]
    let u = Vector::from(vec![1., 2., 3.]);
    let v = Vector::from(vec![4., 5., 6.]);
    println!("{}\n", cross_product(&u, &v));
    // [-3.]
    // [6.]
    // [-3.]
    let u = Vector::from(vec![4., 2., -3.]);
    let v = Vector::from(vec![-2., -5., 16.]);
    println!("{}\n", cross_product(&u, &v));
    // [17.]
    // [-58.]
    // [-16.]
}
