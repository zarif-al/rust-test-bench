mod add_employee;
mod add_department;
mod view_employees;

use std::collections::HashMap;
use add_employee::add_employee;
use add_department::add_department;
use view_employees::view_employees;
use crate::custom_input::take_numberical_input;

pub fn employee_management_system() {
    let mut employees: HashMap<String, String> = HashMap::new();
    let mut departments: Vec<String> = [].to_vec();
    let mut user_input: u32;

    loop {
        println!("\nDo you want to start with a seed data?\n");
        println!("0. Yes");
        println!("1. No");

        user_input = take_numberical_input(2);

        // Seed
        if user_input == 0 {
            let departments_seed = [
                String::from("Software Development"),
                String::from("Design"),
                String::from("Artificial Intelligence"),
            ];

            let employee_seed = HashMap::from([
                (String::from("Sam Smith"), departments_seed[0].to_string()),
                (String::from("Lionel Messi"), departments_seed[1].to_string()),
                (String::from("Cristiano Ronaldo"), departments_seed[0].to_string()),
                (String::from("Leo Brandt"), departments_seed[2].to_string()),
            ]);

            departments.append(&mut departments_seed.to_vec());

            employees.extend(employee_seed);

            println!("\nSeed Successfull!\n");

            break;
        }

        break;
    }

    loop {
        println!("\nWelcome to the Employee Management System\n");
        println!("0. View employees");
        println!("1. Add a new employee");
        println!("2. Add a new department");
        println!("3. Exit");

        user_input = take_numberical_input(4);

        // View all employees
        if user_input == 0 {
            view_employees(&departments, &employees);
            continue;
        }

        // Add a new employee
        if user_input == 1 {
            add_employee(&mut employees, &departments);
            continue;
        }

        // Add a department
        if user_input == 2 {
            add_department(&mut departments);
            continue;
        }

        // Exit
        if user_input == 3 {
            return;
        }
    }
}