use crate::core::Complex;

pub fn test_15() {
    let c = Complex::new(3., 2.);
    let d = Complex::new(5., -3.);
    println!("{}\n", c + d);
    // 8 - i
    println!("{}\n", c - d);
    // -2 + 5i
    println!("{}\n", c * d);
    // 21 + i
    println!("{}\n", c / d);
    // 0.2647058823529412 + 0.5588235294117647i
}
