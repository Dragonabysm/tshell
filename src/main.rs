pub use parser::searcher::matcher;
use serde_json;
use serde_json::Value;
use std::fs::read_to_string;
use std::io::stdin;
use std::io::stdout;
use std::io::Write;
use std::env::current_dir;

mod parser;

fn main() {
    let config = get_json_config();
    let mut exec = 0;

    loop {
        exec += 1;

        let mut output = stdout();
        let current_dir = current_dir().unwrap();

        if config["print.path"].as_bool() == Some(true) {
            println!("\x1b[1;32m[{}]\x1b {}", exec, current_dir.display())
        }

        output.write_all(b"\x1b[1;35m[TSH]\x1b[0m \x1b[2;36m$ \x1b ");
        output.flush();
        output.write(b"\x1b[0m");
        output.flush();

        let mut input = String::new();

        stdin().read_line(&mut input);

        let input = input.trim().to_lowercase();
        let input_split = input.split(" ");
        let input_split: Vec<&str> = input_split.collect();

        matcher(input_split);

        if input == String::from("exit") {
            break;
        }
    }
}

fn get_json_config() -> Value {
    let json_file =
        read_to_string("C:/Users/abili/Desktop/Higor/rust/tricker/config.json").unwrap();
    let json: Value = serde_json::from_str(&json_file).unwrap();

    json
}
