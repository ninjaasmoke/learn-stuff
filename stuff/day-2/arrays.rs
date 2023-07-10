use std::mem;

// This function borrows a slice.
fn analyze_slice(slice: &[i32]) {
    println!("First element of the slice: {}", slice[0]);
    println!("The slice has {} elements \n", slice.len());
}

fn main() {
    println!("Hello arrays!\n");

    // Fixed-size array (type signature is superfluous).
    let xs: [i32; 5] = [1, 2, 3, -4, 5];
    println!("xs = {:?}\n", xs);

    // all elements initialsed to same value!
    let y: [i32; 100] = [0; 100];
    println!("y = {:?}\n", y);

    // properties

    // access
    // println!("xs[0] = {}", xs[0]);

    // out of bound access -> compile error
    // println!("xs[100] = {}", xs[100]);

    // length
    // println!("{}", y.len());

    // array is stack allocated in rust
    // println!("Array `xs` occupies, {} bytes", mem::size_of_val(&xs));
    // println!("Array `y` occupies, {} bytes", mem::size_of_val(&y));

    // Arrays can be automatically borrowed as slices.
    println!("Borrow the whole array as a slice.");
    analyze_slice(&xs);

    println!("Borrow a section of the array as a slice.");
    analyze_slice(&y[1..99]); // include 2 nd element, exclude 100th element

    let empty_array: [u32; 0] = [];
    assert_eq!(&empty_array, &[]);
    assert_eq!(&empty_array, &[][..]); // Same but more verbose

    
    // .get
    for i in 0..xs.len() + 1 { // Oops, one element too far!
        match xs.get(i) {
            Some(xval) => println!("{}: {}", i, xval),
            None => println!("Slow down! {} is too far!", i),
        }
    }
}
