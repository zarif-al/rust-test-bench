mod mean_and_median;
mod pig_latin;
use crate::mean_and_median::{ MeanAndMedian, get_mean_and_median };
use crate::pig_latin::get_pig_latin_translation;

fn main() {
    /*     let result: MeanAndMedian = get_mean_and_median([4, 1, 2, 3, 6, 8, 7]);
    println!("#########");
    println!("Mean: {}, Median: {}", result.mean, result.median);
    println!("#########"); */
    let text_one = String::from("Apple");
    println!("{:?}", get_pig_latin_translation(&text_one))
}