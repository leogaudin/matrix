use crate::core::Matrix;

pub fn test_11() {
    let u = Matrix::from(vec![
        vec![1., -1.],
        vec![-1., 1.]
    ]);
    println!("{}\n", u.determinant());
    // 0.0
    let u = Matrix::from(vec![
        vec![2., 0., 0.],
        vec![0., 2., 0.],
        vec![0., 0., 2.]
    ]);
    println!("{}\n", u.determinant());
    // 8.0
    let u = Matrix::from(vec![
        vec![8., 5., -2.],
        vec![4., 7., 20.],
        vec![7., 6., 1.]
    ]);
    println!("{}\n", u.determinant());
    // -174.0
    let u = Matrix::from(vec![
        vec![8., 5., -2., 4.],
        vec![4., 2.5, 20., 4.],
        vec![8., 5., 1., 4.],
        vec![28., -4., 17., 1.],
    ]);
    println!("{}\n", u.determinant());
    // 1032
    let u = Matrix::from(vec![
        vec![8., 0., 7., 2., 0.],
        vec![0., 7., 0., 3., 0.],
        vec![7., 0., 0., 4., 0.],
        vec![0., 8., 0., 0., 1.],
        vec![0., 0., 8., 0., 0.],
    ]);
    println!("{}\n", u.determinant());
    // 1008
}
