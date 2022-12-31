use std::collections::HashMap;
use std::io;
use crate::numerical_input::take_numberical_input;
// TODO: Create two specific functions.
// view_all_employees -> View all employees by departments sorted.
// view_employees_by_department -> View employees of a specified department.

pub fn view_employees(departments: &Vec<String>, employees: &HashMap<String, String>) {
    let user_input: u32;

    println!("\nHow would like to view the employee list?\n");
    println!("0. All employees in a specific department.");
    println!("1. All employees by department.");

    user_input = take_numberical_input(1);

    if user_input == 0 {
        view_employees_by_department(departments, employees);
    }
}

fn view_employees_by_department(departments: &Vec<String>, employees: &HashMap<String, String>) {
    let department_length = i32::try_from(departments.len()).unwrap_or(0);
    let department_selection: i32;
    let mut input = String::new();

    println!("\nPlese select a department.");
    for (i, el) in departments.iter().enumerate() {
        println!("{}: {}", i, el);
    }

    loop {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        // Reject invalid number input and restart current loop
        let user_input: i32 = match input.trim().parse() {
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

        department_selection = user_input;
        break;
    }

    for (k, v) in employees {
        if &v == &departments.get(department_selection as usize).unwrap_or(&String::from("Null")) {
            println!("{}", k);
        }
    }
}