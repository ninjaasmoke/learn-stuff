/*
    printing is done by macros (default behaviour)
*/

fn main() {

    // print_stuff();
    // print_named_args();
    // format_char_args();
    pad_stuff();
}

#[allow(dead_code)] // allows dead code, dk how it works yet under the hood (XD)
fn print_stuff() {
    println!("\n****____PRINT____****\n\n");
    let f = format!("format! Write formatted text to string!\n");
    print!("{}", f); // {} => will be replaced by stringified `arguements`

    print!("print is same as format but also printed to console (io::stdout)\n");
    println!("same as print!, but new line appended! coool!");

    eprint!("print to stderr\n");
    eprintln!("print to stderr with new line");
}

#[allow(dead_code)] // is an attribute that disables the `dead_code` lint
fn print_named_args() {
    print!("\n\n*****_____arguments_____*****\n\n\n");
    println!("{name} likes to play {activity}", activity="valorant", name="Dummu"); // non positional, named arguments
    println!("{0} {1} {0}", "zero", "one"); // positional arguments
}

#[allow(dead_code)]
fn format_char_args() {
    println!("\n\n*****_____format_char_____*****\n\n");
    println!("Base 10:                          {:}", 10);
    println!("Base 2 (binary):                  {:b}", 10);
    println!("Base 8:                           {:o}", 10);
    println!("Base 16 (hexadecimal):            {:x}", 10);
    println!("Base 16 (hexadecimal):            {:X}", 10);
}

/*
    Traits:
            - ``, which uses the `Display` trait
            - `?`, which uses the `Debug` trait
            - `e`, which uses the `LowerExp` trait
            - `E`, which uses the `UpperExp` trait
            - `o`, which uses the `Octal` trait
            - `p`, which uses the `Pointer` trait
            - `b`, which uses the `Binary` trait
            - `x`, which uses the `LowerHex` trait
            - `X`, which uses the `UpperHex` trait
*/


fn pad_stuff() {
    println!("\n\n*****_____padding_____*****\n\n");

    println!("{data:>10}", data="a"); // right aligned by 10 spaces
    println!("{data:<10} here", data="a"); // left aligned data by 10 spaces in between

    println!("{data:0<5}", data=1); // 10000
    println!("{number:0>width$}", number=1, width=5); // 00001

    let number: f64 = 1.0;
    let width: usize = 5;
    println!("{number:>width$}");

    // println!("This struct `{}` won't print...", Structure(3)); 
    // the trait `std::fmt::Display` is not implemented for `Structure`

    let pi = 3.141592;
    println!("PI is roughly: {:.3}", pi);

}

/*
    Only types that implement fmt::Display can be formatted with `{}`. User-defined types do not implement fmt::Display by default.
*/

#[allow(dead_code)]
struct Structure(i32); // creating a struct

