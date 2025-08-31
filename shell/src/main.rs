use std::io::{self, Write};
use ctrlc;
use evaluating::evaluate;
// use std::io::stdin;
// use termion::{event::Key, input::TermRead};


fn main() {

    ctrlc::set_handler(move || {
        println!("");
    }).expect("Error setting Ctrl-C handler");

    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        match read_input(&mut input) {
            Ok(0) =>  std::process::exit(0),
            Ok(_) => evaluate(&input),
            Err(e) => eprint!("{}", e), 
        }
    }
}

fn read_input(input: &mut String) -> Result<usize, std::io::Error> {
    let mut i : Result<usize, std::io::Error>;
    loop {
        let mut line = String::new();
        i = io::stdin().read_line(&mut line);
        if let Ok(x) = i && x == 0 {
            std::process::exit(0);
        }
        input.push_str(&line);
        if quotes_even(&input) {
            break;
        }else{
            print!(">");
            io::stdout().flush().unwrap();
        }
    }
    i
}

fn quotes_even(input: &str) -> bool {
    let double_quotes = input.matches('"').count();
    let single_quotes = input.matches('\'').count();

    double_quotes % 2 == 0 && single_quotes % 2 == 0
}


// fn main() {
//     let stdin = stdin();
//     for c in stdin.keys() {
//         match c.unwrap() {
//             Key::Up => {

//             }
//             Key::Down => {

//             }
//             _ => {}
//         }
//     }
// }
