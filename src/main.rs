mod core;
mod test;
use test::*;

use colored::Colorize;

fn main() {
    let tests: Vec<(&str, fn())> = vec![
        ("00 - Add, Subtract and Scale", test_00),
        ("01 - Linear combination", test_01),
        ("02 - Linear interpolation", test_02),
        ("03 - Dot product", test_03),
        ("04 - Norm", test_04),
        ("05 - Cosine", test_05),
        ("06 - Cross product", test_06),
        ("07 - Matrix multiplication", test_07),
        ("08 - Trace", test_08),
        ("09 - Transpose", test_09),
        ("10 - Row echelon form", test_10),
        ("11 - Determinant", test_11),
        ("12 - Inverse", test_12),
        ("13 - Rank", test_13),
        ("14 - Projection matrix", test_14),
    ];

    for (title, test) in tests {
        println!("Test {}", title.bold());
        test();
    }
}
