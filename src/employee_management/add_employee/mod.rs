use std::convert::TryFrom;
use std::collections::HashMap;
use std::io;

pub fn add_employee(employees: &mut HashMap<String, String>, departments: &Vec<String>) -> u32 {
    // Declare variables
    let department_length = i32::try_from(departments.len()).unwrap_or(0);
    let mut employee_name = String::new();
    let mut department_input = String::new();
    let department_index: i32;

    // Ask for user input
    println!("\nPlease enter the employee name or enter 0 to exit.\n");

    io::stdin().read_line(&mut employee_name).expect("Failed to read line");

    // Check if the user has entered a valid number
    let exit_command: u32 = match employee_name.trim().parse() {
        Err(_) => { 1 }
        Ok(num) => { num }
    };

    // If the number is 0 then exit with 0
    if exit_command == 0 {
        return 0;
    }

    // Check if employee exists
    let existing_employee = employees.get(&employee_name).unwrap_or(&String::from("None"));

    if existing_employee == "None" {
        println!("\nThis employee already exists!\n");
        return 0;
    }

    // Start a loop to take deparment index input
    loop {
        // Ask the user to select a department.
        println!("\nPlese select a department.");

        // List available departments
        for (i, el) in departments.iter().enumerate() {
            println!("{}: {}", i, el);
        }

        department_input.clear();
        io::stdin().read_line(&mut department_input).expect("Failed to read line");

        // Reject invalid number input and restart current loop
        let user_input: i32 = match department_input.trim().parse() {
            Err(_) => {
                println!("\nPlease input a valid number\n");
                continue;
            }
            Ok(num) => { num }
        };

        // Reject invalid range input and restart the loop
        if user_input >= department_length {
            println!("\nPlease input a valid number\n");
            continue;
        }

        // Assign a valid department index and break loop.
        department_index = user_input;
        break;
    }

    // Insert employee into employees Hash Map
    employees.insert(
        employee_name.trim().to_string(),
        departments
            .get(department_index as usize)
            .unwrap_or(&String::from("None"))
            .to_string()
    );

    // Exit with 1
    1
}