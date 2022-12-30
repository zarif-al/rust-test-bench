use std::collections::HashMap;
use std::io;
use add_employee::add_employee;

mod add_employee;

pub fn employee_management_system() {
    let mut employees: HashMap<String, String> = HashMap::new();
    let departments: Vec<String> = [
        "Department 1".to_string(),
        "Department 2".to_string(),
    ].to_vec();
    let mut input = String::new();

    loop {
        //TODO: Add implmentation for adding a new department.
        //TODO: Add implmentation for modifying existing employees.
        println!("\nWelcome to the Employee Management System\n");
        println!("1. View employees");
        println!("2. Add a new employee");
        println!("3. Exit");

        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let options_input: u32 = match input.trim().parse() {
            Err(_) => {
                println!("\nPlease input a number\n");
                continue;
            }
            Ok(num) => { num }
        };

        if options_input == 3 {
            break;
        } else if options_input > 3 {
            println!("\nPlese enter a valid option.\n");
            continue;
        }

        // User has entered a valid option

        // Add a new employee
        let add_employee_result = add_employee(&mut employees, &departments);

        if add_employee_result == 0 {
            println!("\nExit Employee Addition.\n");
            continue;
        } else {
            println!("\nYou have added a new employee.");
            println!("Employees: {:?}\n", employees);
            continue;
        }
    }
}