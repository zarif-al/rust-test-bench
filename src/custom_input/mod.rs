use std::io;

pub fn take_string_input() -> String {
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Failed to read line");
    input = input.trim().to_string();

    // Check if the user has entered a valid number
    let exit_command: u32 = match input.parse() {
        Err(_) => { 1 }
        Ok(num) => { num }
    };

    // If the number is 0 then return empty string
    if exit_command == 0 {
        return String::from("");
    }

    return input;
}

pub fn take_numberical_input(max: u32) -> u32 {
    let mut input = String::new();
    let mut user_input: u32;

    // Run a loop until user enters a valid input within range.
    loop {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        input = input.trim().to_string();

        user_input = match input.trim().parse() {
            Err(_) => {
                println!("\nPlease input a valid number\n");
                continue;
            }
            Ok(num) => { num }
        };

        if user_input > max {
            println!("\nPlease input a valid number\n");
            continue;
        }

        break;
    }

    return user_input;
}