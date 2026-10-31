//!
//! Misc. utility functions
//!

use std::io;

// Get an int from user input
pub fn get_input_int() -> isize {
    let mut input = String::new();
    loop {
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        if let Ok(i) = input.trim().parse::<isize>() {
            return i;
        }
        eprintln!("Failed to parse as integer: {:?}", input);
        input.clear();
    }
}

// Get a bool from user input
pub fn get_bool_input() -> bool {
    let mut input = String::new();
    loop {
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        match input.trim() {
            "t" => return true,
            "f" => return false,
            _ => {
                eprintln!("Expected t or f");
                input.clear();
            }
        };
    }
}

// Need a crate for this
use rand;

// Get a random bool
pub fn rand_bool() -> bool {
    rand::random()
}
