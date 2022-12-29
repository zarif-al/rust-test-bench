pub struct MeanAndMedian {
    pub mean: f64,
    pub median: f64,
}

pub fn get_mean_and_median(list: [i64; 7]) -> MeanAndMedian {
    // Convert array to vector.
    let mut number_vector = list.to_vec();

    number_vector.sort();

    let length = number_vector.len();

    let median;
    if length % 2 != 0 {
        let first_term = number_vector[length / 2 - 1];
        let second_term = number_vector[length / 2];
        median = ((first_term + second_term) as f64) / 2.0;
    } else {
        median = number_vector[length / 2] as f64;
    }

    let mut sum = 0;
    for n in 0..length {
        sum = number_vector[n] + sum;
    }

    return MeanAndMedian { mean: (sum as f64) / (length as f64), median: median };
}