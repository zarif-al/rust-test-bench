mod employee_management;
use crate::employee_management::employee_management_system;

/* mod mean_and_median;
mod pig_latin; */

/* use crate::mean_and_median::{ MeanAndMedian, get_mean_and_median };
use crate::pig_latin::get_pig_latin_translation; */

fn main() {
    println!("Employee Management System");
    employee_management_system();

    /*
    let result: MeanAndMedian = get_mean_and_median([4, 1, 2, 3, 6, 8, 7]);
    println!("");
    println!("Mean, Median and Mode");
    println!("Mean: {}, Median: {}", result.mean, result.median);
    */

    /*
    let text_one = String::from("Apple");
    let text_two: String = String::from("first");
    println!("Pig Latin Translation");
    println!("{:?}", get_pig_latin_translation(&text_one));
    println!("{:?}", get_pig_latin_translation(&text_two));
    */
}