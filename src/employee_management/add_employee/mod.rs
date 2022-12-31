use std::convert::TryFrom;
use std::collections::HashMap;
use crate::custom_input::{ take_numberical_input, take_string_input };

pub fn add_employee(employees: &mut HashMap<String, String>, departments: &Vec<String>) {
    // Declare variables
    let department_length = u32::try_from(departments.len()).unwrap_or(0);
    let mut user_input: u32;

    // Ask for user input
    println!("\nPlease enter the employee name or enter 0 to exit.\n");

    let employee_name = take_string_input();

    // Check if employee name is empty
    if employee_name.is_empty() {
        return;
    }

    // Check if employee exists
    let binding = &String::from("None");
    let existing_employee = employees.get(&employee_name).unwrap_or(binding);

    if existing_employee != "None" {
        // Employee already exists. Ask user if they would like to modify existing user.
        println!("\nThis employee already exists! Would you like to modify their department?\n");
        println!("0. Yes");
        println!("1. No");

        user_input = take_numberical_input(1);

        // If user selects "No" then exit.
        if user_input == 1 {
            return;
        }
    }

    // List available departments
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

    // Insert employee into employees Hash Map
    employees.insert(employee_name.trim().to_string(), selected_department);
}