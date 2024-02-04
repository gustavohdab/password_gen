// This program generates a random password of a given length.
// The password is a mix of uppercase letters, lowercase letters, numbers, and symbols.
// The generated password is copied to the clipboard automatically.
// The generated password is printed to the console.
// The length of the password can be configured via a command line argument.
// The password is generated using a cryptographically secure random number generator.

// TODO: Implement a command to view the history of all generated passwords.
//       Instead of entering a number, allow the user to enter the command "/history" to see all generated passwords.
//       Additionally, support the command "/history <name>" to view a specific password.
// TODO: Prompt the user to provide a identifier for each generated password.
//       This will help in managing the password history and easily identifying passwords later.

use clipboard_win::{formats, set_clipboard};
use rand::Rng;
use std::env;
use std::io::{self, Write};
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 1 {
        writeln!(io::stderr(), "Usage: password_gen").unwrap();
        process::exit(1);
    }
    let length: usize = ask_password_length();
    let password: String = generate_password(length);
    println!("{}", password);
    set_clipboard(formats::Unicode, &password).unwrap();
}

// Asks the user to enter the length of the password and returns it.
fn ask_password_length() -> usize {
    let mut input: String = String::new();
    println!("Enter the length of the password:");
    io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}

// Generates a password of the specified length.
fn generate_password(length: usize) -> String {
    let mut rng: rand::prelude::ThreadRng = rand::thread_rng();
    let mut password: String = String::new();
    for _ in 0..length {
        let choice: i32 = rng.gen_range(0..4);
        match choice {
            0 => password.push(rng.gen_range(48_u8..58_u8) as char), // Numbers (ASCII 48-57)
            1 => password.push(rng.gen_range(65_u8..91_u8) as char), // Uppercase letters (ASCII 65-90)
            2 => password.push(rng.gen_range(97_u8..123_u8) as char), // Lowercase letters (ASCII 97-122)
            3 => password.push(rng.gen_range(33_u8..48_u8) as char),  // Symbols (ASCII 33-47)
            _ => panic!("Invalid choice"),
        }
    }
    password
}

// Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_password() {
        let password: String = generate_password(10);
        assert_eq!(password.len(), 10);
    }
}
