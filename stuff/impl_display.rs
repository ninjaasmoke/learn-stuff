use std::fmt;

// here, i import `fmt` module from the `std` lib using `use` keyword

struct SomeStruct(f64);

impl fmt::Display for SomeStruct {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
        // `write!` uses syntax which is very similar to `println!`.
    }
}

#[derive(Debug)] // deriving Debug to contrast with fmt::Display
struct MinMax(i64, i64);

impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({} {})", self.0, self.1)
    }
}

// structure with field names
#[derive(Debug)]
struct Point2D {
    x: f64,
    y: f64,
}

impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

// test struct!
#[derive(Debug)]
struct Complex {
    real: f64,
    imag: f64,
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{0} + {1}i", self.real, self.imag)
    }
}

fn main() {
    let ss = SomeStruct(69.42);
    println!("{}", ss);

    let minmax = MinMax(0, 14);

    println!("\n\nCompare structures:");

    println!("Display: {}", minmax);
    println!("Debug: {:?}", minmax);

    let big_range = MinMax(-300, 300);
    let small_range = MinMax(-3, 3);

    println!(
        "The big range is {big} and the small is {small}",
        small = small_range,
        big = big_range
    );

    let point = Point2D { x: 3.3, y: 7.2 };

    println!("\n\nCompare points:");
    println!("Display: {}", point);
    println!("Debug: {:?}", point);

    let complex_num = Complex { real: 3.3, imag: 7.2 };

    println!("\n\nCustom complex struct!");
    println!("Display:  {}", complex_num);
    println!("Debug:    {:?}", complex_num);
}
