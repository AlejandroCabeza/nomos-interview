use std::io::{self, stdin};
use crate::errors::Error;

pub fn clear_screen() {
    print!("\x1Bc");
}

pub fn get_input() -> Result<String, io::Error> {
    let mut input: String = String::new();
    stdin().read_line(&mut input).map(|_| input)
}

pub fn press_enter_to_continue() {
    println!("> Press Enter to continue <");
    let _ = get_input();
}

pub enum ContinueCommand {
    Continue,
    Exit
}

pub fn continue_or_exit() -> Result<ContinueCommand, Error> {
    println!("> Press Enter to continue, or type \"exit\" to quit <");
    match get_input() {
        Ok(input) => {
            if input == "exit" {
                Ok(ContinueCommand::Exit)
            }
            else {
                Ok(ContinueCommand::Continue)
            }
        }
        Err(error) => Err(Error::Input(error))
    }
}
