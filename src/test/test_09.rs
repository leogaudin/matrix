use crate::core::Matrix;

pub fn test_09() {
    let mut u = Matrix::from(vec![vec![1., 2.], vec![3., 4.]]);
    u.transpose();
    println!("{}\n", u);
    // [1., 3.]
    // [2., 4.]
    let mut u = Matrix::from(vec![vec![1., 2.], vec![3., 4.], vec![5., 6.]]);
    u.transpose();
    println!("{}\n", u);
    // [1., 3., 5.]
    // [2., 4., 6.]
}
