// Topic: User input
//
// Requirements:
// * Verify user input against pre-defined keywords
// * The keywords represent possible power options for a computer:
//   * Off
//   * Sleep
//   * Reboot
//   * Shutdown
//   * Hibernate
// * If the user enters one of the keywords, a message should be printed to
//   the console indicating which action will be taken
//   * Example: If the user types in "shutdown" a message should display such
//     as "shutting down"
// * If the keyword entered does not exist, an appropriate error message
//   should be displayed
//
// Notes:
// * Use an enum to store the possible power states
// * Use a function with a match expression to print out the power messages
//   * The function should accept the enum as an input
// * Use a match expression to convert the user input into the power state enum
// * The program should be case-insensitive (the user should be able to type
//   Reboot, reboot, REBOOT, etc.)

use std::io;

#[derive(Debug)]
enum Action {
    Off,
    Sleep,
    Reboot,
    Shutdown,
    Hibernate,
}

fn get_input() -> io::Result<String> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    Ok(buffer.trim().to_owned())
}

fn print_computer_state(state: &Action) {
    match state {
        Action::Off => println!("off"),
        Action::Sleep => println!("sleep state"),
        Action::Reboot => println!("rebooting"),
        Action::Shutdown => println!("shutting down"),
        Action::Hibernate => println!("hibernate state"),
    }
    // println!("next state = {:?}", state);
}

fn main() {
    match get_input() {
        Ok(input) => {
            let input_lowercased = input.to_lowercase();
            match input_lowercased {
                "off".to_owned() => print_computer_state(&Action::Off),
                "sleep".to_owned() => print_computer_state(&Action::Sleep),
                "reboot".to_owned() => print_computer_state(&Action::Reboot),
                "shutdown".to_owned() => print_computer_state(&Action::Shutdown),
                "hibernate".to_owned() => print_computer_state(&Action::Hibernate),
                _  => return Err("state does not exist".to_owned()),
            }
        }
        Err(e) => println!("error: {:?}", e),
    }
}
