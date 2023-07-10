// Exercise.

use std::fmt::{ Formatter, Display, Result };

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

impl Display for Matrix {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let &Matrix(a, b, c, d) = &self;
        write!(f, "({}, {})\n({}, {})", a, b, c, d)
    }
}

fn transpose(m: &Matrix) -> Matrix {
    let &Matrix(a, b, c, d) = m;
    Matrix(a, c, b, d)
}

fn main() {
    let matrix: Matrix = Matrix(1.1, 1.2, 2.1, 2.2);

    println!("Debug:\n{:?}\n", matrix);
    println!("Display(Matrix):\n{}\n", matrix);
    println!("Transpose:\n{}\n", transpose(&matrix));
}
