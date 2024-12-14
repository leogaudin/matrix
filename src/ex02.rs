#[path = "./structs.rs"]
mod structs;
use structs::{Matrix, Vector};

pub fn lerp<
    V: std::fmt::Display
        + std::ops::Add<Output = V>
        + std::ops::Sub<Output = V>
        + std::ops::Mul<f32, Output = V>
>(
    u: V,
    v: V,
    t: f32,
) -> V {
    return u * (1. - t) + v * t;
}

pub fn test_02() {
    println!("{}\n", lerp(0., 1., 0.));
    // 0.0
    println!("{}\n", lerp(0., 1., 1.));
    // 1.0
    println!("{}\n", lerp(0., 1., 0.5));
    // 0.5
    println!("{}\n", lerp(21., 42., 0.3));
    // 27.3
    println!(
        "{}\n",
        lerp(
            Vector::from(vec![2., 1.]),
            Vector::from(vec![4., 2.]),
            0.5
        )
    );
    // [2.6]
    // [1.3]
    println!(
        "{}\n",
        lerp(
            Matrix::from(vec![vec![2., 1.], vec![3., 4.]]),
            Matrix::from(vec![vec![20., 10.], vec![30., 40.]]),
            0.5
        )
    );
    // [[11., 5.5]
    // [16.5, 22.]]
}
