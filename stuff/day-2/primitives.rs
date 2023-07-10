fn main() {
    // let i: isize = 10;
    // println!("Hello primitive, {}", i);

    // // Integer addition
    // println!("1 + 2 = {}", 1u32 + 2);

    // // Integer subtraction
    // println!("1 - 2 = {}", 1i32 - 2);

    // // Scientific notation
    // println!("1e4 is {}, -2.5e-3 is {}", 1e4, -2.5e-3);

    // // Short-circuiting boolean logic
    // println!("true AND false is {}", true && false);
    // println!("true OR false is {}", true || false);
    // println!("NOT true is {}", !true);

    // Bitwise operations 
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101); // 0b is binary as 0x is hex (used for colors, remember?)
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5); // 1 = 1 in binary, left shift 5 -> 0B10000 =0d32
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2); // wah

    println!("{}", 0b101); // decimal 5

    // Use underscores to improve readability!
    println!("One million is written as {}", 1_000_000u32);
}
