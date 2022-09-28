use std::io;
use math_calc::{ErrorKind, parse_str};


fn main() {
    println!("met- uh i mean math");

    loop {

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("failed to read line");

        match input.trim() {
            "exit" => std::process::exit(0),
            _ => {}
        }

           let output: Result<Vec<i32>, ErrorKind> = parse_str(&input);

           println!("{:?}", output);
    }
}




