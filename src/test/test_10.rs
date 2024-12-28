use crate::core::Matrix;

pub fn test_10() {
    let u = Matrix::from(vec![vec![1., 0., 0.], vec![0., 1., 0.], vec![0., 0., 1.]]);
    println!("{}\n", u.row_echelon());
    // [1.0, 0.0, 0.0]
    // [0.0, 1.0, 0.0]
    // [0.0, 0.0, 1.0]
    let u = Matrix::from(vec![vec![1., 2.], vec![3., 4.]]);
    println!("{}\n", u.row_echelon());
    // [1.0, 0.0]
    // [0.0, 1.0]
    let u = Matrix::from(vec![vec![1., 2.], vec![2., 4.]]);
    println!("{}\n", u.row_echelon());
    // [1.0, 2.0]
    // [0.0, 0.0]
    let u = Matrix::from(vec![
        vec![8., 5., -2., 4., 28.],
        vec![4., 2.5, 20., 4., -4.],
        vec![8., 5., 1., 4., 17.],
    ]);
    println!("{}\n", u.row_echelon());
    // [1.0, 0.625, 0.0, 0.0, -12.1666667]
    // [0.0, 0.0, 1.0, 0.0, -3.6666667]
    // [0.0, 0.0, 0.0, 1.0, 29.5 ]
    let u = Matrix::from(vec![
        vec![0., 0., 7., 2.],
        vec![0., 7., 0., 3.],
        vec![7., 0., 0., 4.],
    ]);
    println!("{}\n", u.row_echelon());
}
