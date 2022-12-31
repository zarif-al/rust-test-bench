use std::io;

pub fn add_department(departments: &mut Vec<String>) {
    // Take user input
    println!("Please enter the new department name:");
    let mut user_input = String::new();

    io::stdin().read_line(&mut user_input).expect("Failed to read line");
    user_input = user_input.trim().to_string();

    /*
    If department exists then return otherwise add it to departments vector
	and then return.
	*/
    match departments.iter().find(|x| x == &&user_input) {
        Some(x) => {
            println!("\n{} alredy exists!\n", x);
            return;
        }
        None => {
            println!("\n{}, has been successfully added!\n", user_input);
            departments.push(user_input);
            return;
        }
    }
}