use crate::core::Matrix;

pub fn projection(fov: f32, ratio: f32, near: f32, far: f32) -> Matrix<f32> {
    let mut projection_matrix: Matrix<f32> = Matrix::from(
        vec![vec![0.0; 4]; 4]
    );

    let tangent: f32 = (fov.to_radians() / 2.0).tan();
    let top: f32 = near * tangent;
    let right: f32 = top * ratio;

    projection_matrix.set(0, 0, near / right);
    projection_matrix.set(1, 1, near / top);
    projection_matrix.set(2, 2, -(far + near) / (far - near));
    projection_matrix.set(3, 2, -(2.0 * far * near) / (far - near));
    projection_matrix.set(2, 3, -1.0);

    return projection_matrix;
}

pub fn test_14() {
    let fov = 100.0;
    let ratio = 3. / 2.;
    let near = 2.0;
    let far = 50.0;
    let projection_matrix = projection(fov, ratio, near, far);

    let string = projection_matrix.to_string();
    let string = string.replace("[", "");
    let string = string.replace("]", "");
    println!("{}\n", string);
    std::fs::write("proj", string).unwrap();
}
