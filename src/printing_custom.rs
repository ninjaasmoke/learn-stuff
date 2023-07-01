fn main() {
    // hello

    // print_user_created_struct();
    pretty_printing_in_rust();
}

#[allow(dead_code)]
struct Unprintable(i32);

#[derive(Debug)]
struct DebugPrintable(i32);

#[derive(Debug)]
struct Structure(u64);

#[derive(Debug)]
struct DeepStruct(Structure);

#[allow(dead_code)]
fn print_user_created_struct() {
    // println!("{:?}", Unprintable(3)); 
    // `Unprintable` cannot be formatted using `{:?}`
    // the trait `Debug` is not implemented for `Unprintable`

    println!("Print custom: {:?}", DebugPrintable(10));

    // deep debug print;
    println!("Deeeep: {:?}", DeepStruct(Structure(10)));

    // Printing with `{:?}` is similar to with `{}`.
    println!("{:?} is my bday.", 14);
    println!("{1:?} {0:?} is the {p:?} name.",
            "Kohli",
            "Virat",
            p="player's");

    // The problem with `derive` is there is no control over how
    // the results look. What if I want this to just show a `7`?
    println!("Now {:?} will print!", DeepStruct(Structure(7)));
}


#[derive(Debug)]
struct Human<'a> {
    name: &'a str,
    age: u8,
    occupation: &'a str
}

fn pretty_printing_in_rust() {
    // {:#?}

    let name = "Nit";
    let age = 23;
    let work = "Engineer"; 
    let occupation = "Engineer"; 

    // let nit = Human {name, age, work}; // err: does not accept different name work
    let nit = Human {name: name, age, occupation:work}; // works
    // or
    let nith = Human {name: name, age, occupation};


    println!("{:#?}", nit);
    println!("{:#?}", nith);
    
    
    println!("{}", nith.name);
}