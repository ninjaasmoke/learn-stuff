use std::fmt::{ Formatter, Display, Result };

fn reverse(pair: (i32, bool)) -> (bool, i32) {
    let (int_param, bool_param) = pair;
    (bool_param, int_param)

    /* 
    or
    `(pair.1, pair.0)`
    
    or 
    `return (pair.1, pair.0);`
    */
}

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

impl Display for Matrix {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let &Matrix(a, b, c, d) = &self;
        write!(f, "({} {})\n({} {})", a, b, c, d)
    }
}

fn transpose_matrix(m: &Matrix) -> Matrix {
    let &Matrix(a, b, c, d) = m;
    Matrix(a, c, b, d)
}

fn main() {
    let pair = (1, true);
    println!("{:?}", reverse(pair));

    // A tuple with a bunch of different types.
    let long_tuple = (1u8, 2u16, 3u32, 4u64, -1i8, -2i16, -3i32, -4i64, 0.1f32, 0.2f64, 'a', true);

    println!("Long tuple first value: {}", long_tuple.0);
    println!("Long tuple second value: {}", long_tuple.1);

    // tuples inside tupples

    let tuple_meta = ((4i32, 10u16), 1u64, (u64::MAX, i128::MAX), (u64::MIN, i64::MIN));
    println!("Tuple meta: {:?}", tuple_meta);

    // But long Tuples (more than 12 elements) cannot be printed.
    // let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
    // println!("Too long tuple: {:?}", too_long_tuple);

    // create 1 element tuple, add `,` to separate from `()` literal

    let one_tuple: (i8,) = (1i8,);
    println!("One tuple: {:?}", one_tuple);

    let one_litral = 1u8;
    println!("One literal: {:?}", one_litral);

    let tuple = (1, "hello", 4.5, true);
    // destructing!!
    let (a, b, c, d) = tuple;
    println!("{:?}, {:?}, {:?}, {:?}", a, b, c, d);

    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    // debug
    println!("{:?}", matrix);

    // display
    println!("Matrix:\n{}", matrix);
    println!("Transpose:\n{}", transpose_matrix(&matrix));
}
