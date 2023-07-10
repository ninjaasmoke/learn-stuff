// simple
use std::fmt::{ Formatter, Display, Result };
use std::io;

#[derive(Debug)]
struct Question {
    list: Vec<i32>,
    num: i32,
}

#[derive(Debug)]
struct Answer {
    search_result: bool,
    search_pair: (i32, i32),
    search_number: i32,
}

impl Display for Answer {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let result: &Answer = &self;
        let (a, b): (i32, i32) = result.search_pair;
        write!(f, "Result: {}\n", result.search_result)?;
        write!(f, "Pair: ({} {})\n", a, b)?;
        write!(f, "Number: {}\n", result.search_number)
    }
}

fn main() {
    // TODO: accept list and number from std input

    let question = Question {
        list: vec![1, 2, 3, 4, 5, 6],
        num: 10,
    };

    let mut complement_list: Vec<i32> = vec![];

    for i in &question.list {
        let complement = question.num - i;
        if complement_list.contains(&complement) {
            let answer = Answer {
                search_result: true,
                search_pair: (*i, complement),
                search_number: question.num,
            };
            println!("{}", answer);
            return;
        } else {
            complement_list.push(*i);
        }
    }

    let answer = Answer {
        search_result: false,
        search_pair: (0, 0),
        search_number: question.num,
    };

    println!("{}", answer);
    return;
}
