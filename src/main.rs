pub use parser::searcher::matcher;
use std::env::current_dir;
use std::io::stdin;
use std::io::stdout;
use std::io::Write;

mod funcs;
mod parser;
mod user_assist;

fn main() {
    let mut exec = 0;

    loop {
        exec += 1;

        let mut output = stdout();
        let current_dir = current_dir().unwrap();

        println!("\x1b[1;32m[{}] \x1b {}", exec, current_dir.display());

        output.write_all(b"\x1b[1;35m[TSH]\x1b[0m \x1b[2;36m$ \x1b ").unwrap();
        output.flush().unwrap();
        output.write(b"\x1b[0m").unwrap();
        output.flush().unwrap();

        let mut input = String::new();

        stdin().read_line(&mut input).expect("Error on read input.");

        let input = input.trim().to_lowercase();
        let input_split = input.split(" ");
        let input_split: Vec<&str> = input_split.collect();

        matcher(input_split);

        if input == String::from("exit") {
            break;
        }
    }
}