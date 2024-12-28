use crate::core::Matrix;

pub fn test_08() {
    let u = Matrix::from(vec![vec![1., 0.], vec![0., 1.]]);
    println!("{}\n", u.trace());
    // 2.0
    let u = Matrix::from(vec![vec![2., -5., 0.], vec![4., 3., 7.], vec![-2., 3., 4.]]);
    println!("{}\n", u.trace());
    // 9.0
    let u = Matrix::from(vec![vec![-2., -8., 4.], vec![1., -23., 4.], vec![0., 6., 4.]]);
    println!("{}\n", u.trace());
    // -21.0
}
