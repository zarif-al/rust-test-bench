use std::collections::HashMap;
use crate::custom_input::take_numberical_input;

pub fn view_employees(departments: &Vec<String>, employees: &HashMap<String, String>) {
    let user_input: u32;

    if employees.len() == 0 {
        println!("\nCurrently you have no employees!\n");
        return;
    }

    println!("\nHow would like to view the employee list?\n");
    println!("0. All employees in a specific department.");
    println!("1. All employees by department.");

    user_input = take_numberical_input(1);

    if user_input == 0 {
        view_employees_by_department(departments, employees);
    }

    if user_input == 1 {
        view_all_employees(employees);
    }
}

fn view_employees_by_department(departments: &Vec<String>, employees: &HashMap<String, String>) {
    let department_length = u32::try_from(departments.len()).unwrap_or(0);
    let user_input: u32;

    println!("\nPlese select a department.");
    for (i, el) in departments.iter().enumerate() {
        println!("{}: {}", i, el);
    }

    user_input = take_numberical_input(department_length);

    let binding = String::from("Null");
    let selected_department = departments
        .get(user_input as usize)
        .unwrap_or(&binding)
        .to_string();

    if selected_department == binding {
        println!("\nThere was an error processing your request.\n");
        return;
    }

    for (k, v) in employees {
        if v == &selected_department {
            println!("{}", k);
        }
    }
}

fn view_all_employees(employees: &HashMap<String, String>) {
    let mut employee_vector: Vec<(&String, &String)> = employees.into_iter().collect();

    employee_vector.sort_by_key(|k| k.1);

    for item in employee_vector {
        println!("{}, {}", item.0, item.1);
    }
}