mod structs;
mod ex00;
use ex00::test_00;
mod ex01;
use ex01::test_01;
mod ex02;
use ex02::test_02;
mod ex03;
use ex03::test_03;
mod ex04;
use ex04::test_04;
mod ex05;
use ex05::test_05;
mod ex06;
use ex06::test_06;
mod ex07;
use ex07::test_07;
mod ex08;
use ex08::test_08;
mod ex09;
use ex09::test_09;
mod ex10;
use ex10::test_10;
mod ex11;
use ex11::test_11;
mod ex12;
use ex12::test_12;
mod ex13;
use ex13::test_13;
mod ex14;
use ex14::test_14;

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
