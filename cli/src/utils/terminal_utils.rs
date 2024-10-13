use crate::errors::Error;
use std::io;
use std::io::stdin;

pub fn clear_screen() {
    print!("\x1Bc");
}

pub fn get_input() -> Result<String, io::Error> {
    let mut input: String = String::new();
    stdin().read_line(&mut input).map(|_| input)
}

pub fn get_normalized_input() -> Result<String, io::Error> {
    get_input().map(|input| input.trim().to_lowercase())
}

pub enum ContinueCommand {
    Continue,
    Exit,
}

pub fn continue_or_exit() -> Result<ContinueCommand, Error> {
    println!("> Press Enter to continue, or type \"exit\" to quit <");
    match get_normalized_input() {
        Ok(input) => {
            if input == "exit" {
                Ok(ContinueCommand::Exit)
            } else {
                Ok(ContinueCommand::Continue)
            }
        }
        Err(error) => Err(Error::IO(error)),
    }
}
