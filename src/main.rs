pub use parser::searcher::matcher;
use std::env::current_dir;
use std::io::stdin;
use std::io::stdout;
use std::io::Write;
use std::path::PathBuf;

mod funcs;
mod parser;
mod user_assist;

#[cfg(windows)]
pub fn set_virtual_terminal(use_virtual: bool) -> Result<(), ()> {
    use winapi::{
        shared::minwindef::DWORD,
        um::{
            consoleapi::{GetConsoleMode, SetConsoleMode},
            processenv::GetStdHandle,
            winbase::STD_OUTPUT_HANDLE,
            wincon::ENABLE_VIRTUAL_TERMINAL_PROCESSING,
        },
    };

    unsafe {
        let handle = GetStdHandle(STD_OUTPUT_HANDLE);
        let mut original_mode: DWORD = 0;
        GetConsoleMode(handle, &mut original_mode);

        let enabled = original_mode & ENABLE_VIRTUAL_TERMINAL_PROCESSING
            == ENABLE_VIRTUAL_TERMINAL_PROCESSING;

        match (use_virtual, enabled) {
            (true, false) => {
                SetConsoleMode(handle, ENABLE_VIRTUAL_TERMINAL_PROCESSING | original_mode)
            }
            (false, true) => {
                SetConsoleMode(handle, ENABLE_VIRTUAL_TERMINAL_PROCESSING ^ original_mode)
            }
            _ => 0,
        };
    }

    Ok(())
}

fn main() {
    let mut exec: u16 = 0;
    set_virtual_terminal(true).unwrap();

    loop {
        exec += 1;

        let mut output: std::io::Stdout = stdout();
        let current_dir: PathBuf = current_dir().unwrap();

        println!("\x1b[1;32m[{}] \x1b {}", exec, current_dir.display());

        output.write_all(b"\x1b[1;35m[TSH]\x1b[0m \x1b[2;36m$ \x1b ").unwrap();
        output.flush().unwrap();
        output.write(b"\x1b[0m").unwrap();
        output.flush().unwrap();

        drop(output);
        drop(current_dir);

        let mut input: String = String::new();

        stdin().read_line(&mut input).expect("Error on read input.");

        let input: String = input.trim().to_lowercase();
        let input_split = input.split(" ");
        let input_split: Vec<&str> = input_split.collect();

        if input == String::from("exit") || input == String::from("quit") {
            break;
        }
        matcher(input_split);
    }
}