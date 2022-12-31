use crate::custom_input::take_string_input;

pub fn add_department(departments: &mut Vec<String>) {
    // Take user input
    println!("Please enter the new department name or 0 to exit:");
    let department_name = take_string_input();

    if department_name.is_empty() {
        return;
    }

    /*
    If department exists then return otherwise add it to departments vector
	and then return.
	*/
    match departments.iter().find(|x| x == &&department_name) {
        Some(x) => {
            println!("\n{} alredy exists!\n", x);
            return;
        }
        None => {
            println!("\n{}, has been successfully added!\n", department_name);
            departments.push(department_name);
            return;
        }
    }
}