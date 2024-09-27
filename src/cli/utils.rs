use std::io::{stdin, Error};

pub fn clear_screen() {
    print!("\x1Bc");
}

pub fn get_input() -> Result<String, Error> {
    let mut input: String = String::new();
    stdin().read_line(&mut input).map(|_| input)
}

pub fn press_enter_to_continue() {
    println!("> Press Enter to continue <");
    let _ = get_input();
}
