use std::fmt;

#[derive(Debug)]
struct List(Vec<i32>); // `List` is not a keyword; `Vec` is

/*
impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // write!(f, "{}", self.0) // `Vec<i32>` cannot be formatted with the default formatter

        let vec = &self.0;

        write!(f, "[")?;

        for (count, v) in vec.iter().enumerate() {
            if count != 0 {
                // For every element except the first, add a comma. Use the ? operator to return on errors.
                write!(f, ", ")?;
            }
            write!(f, "{}", v)?;
        }

        write!(f, "]")
    }
}

*/

// variation
impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let vec = &self.0;

        if vec.is_empty() {
            return write!(f, "[]");
        }

        write!(f, "[")?;
        write!(f, "{}", vec[0])?;

        for v in &vec[1..] {
            write!(f, ", {}", v)?;
        }
        write!(f, "]")
    }
}

#[derive(Debug)]
struct ListWithIndex(Vec<i32>);

impl fmt::Display for ListWithIndex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Use the ? operator to return on errors.
        write!(f, "[")?;

        let vec = &self.0; // use the refenrece to the address of the `List`; i assume
        let last_index = vec.len() - 1;
        for (c, v) in vec.iter().enumerate() {
            write!(f, "{}: {}", c, v)?;
            if c != last_index {
                write!(f, ", ")?;
            }
        }
        write!(f, "]")
    }
}




fn main() {
    let list = List(vec![1, 2, 3, 0]);
    println!("Debug:    {:?}", list);
    println!("Display:  {}", list);

    let list_index = ListWithIndex(vec![1, 2, 3]);
    println!("\n\nDebug:    {:?}", list_index);
    println!("Display:  {}", list_index);
}
