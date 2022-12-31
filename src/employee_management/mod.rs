use std::collections::HashMap;
use std::io;
use add_employee::add_employee;
use add_department::add_department;
use view_employees::view_employees;

mod add_employee;
mod add_department;
mod view_employees;

pub fn employee_management_system() {
    let mut employees: HashMap<String, String> = HashMap::new();
    let mut departments: Vec<String> = [
        "Department 1".to_string(),
        "Department 2".to_string(),
    ].to_vec();
    let mut input = String::new();

    loop {
        println!("\nWelcome to the Employee Management System\n");
        println!("0. View employees");
        println!("1. Add a new employee");
        println!("2. Add a new department");
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

        // View all employees
        if options_input == 0 {
            view_employees(&departments, &employees);
            continue;
        }

        // Add a new employee
        if options_input == 1 {
            add_employee(&mut employees, &departments);
            continue;
        }

        // Add a department
        if options_input == 2 {
            add_department(&mut departments);
            continue;
        }
    }
}