mod print;

use crate::print::*;

fn median(values: &mut Vec<i32>) -> f64 {
    values.sort();
    if values.len() % 2 == 0 {
        let left_center = values[values.len() / 2 - 1];
        let right_center = values[values.len() / 2];
        f64::from(left_center + right_center) / 2.0
    } else {
        f64::from(values[values.len() / 2])
    }
}

fn test_median(values: &mut Vec<i32>, expected_median: f64) {
    println!("Values: {:?}", values);
    let actual_median = median(values);
    println!("Expected Median: {expected_median}");
    println!("Actual   Median: {actual_median}");
    if expected_median == actual_median {
        print_pass_msg("Pass");
    } else {
        print_fail_msg(&format!("Fail: {expected_median} != {actual_median}"));
    }
}

use std::collections::HashMap;

fn mode(values: &Vec<i32>) -> Vec<i32> {
    let mut frequencies = HashMap::new();

    let mut max_frequency = 0;
    for value in values {
        let frequency = frequencies.entry(value).or_insert(0);
        *frequency += 1;
        let frequency = *frequency;
        if frequency > max_frequency {
            max_frequency = frequency;
        }
    }

    let mut mode = Vec::new();
    for (value, frequency) in frequencies {
        if frequency == max_frequency {
            mode.push(*value);
        }
    }
    mode.sort();
    mode
}

fn test_mode(values: &Vec<i32>, expected_mode: &Vec<i32>) {
    println!("Values: {:?}", values);
    println!("Expected Mode: {:?}", expected_mode);
    let actual_mode = mode(&values);
    println!("Actual   Mode: {:?}", actual_mode);
    if *expected_mode == actual_mode {
        print_pass_msg("Pass");
    } else {
        print_fail_msg(&format!("Fail: {:?} != {:?}", *expected_mode, actual_mode));
    }
}

fn main() {
    // Median
    test_median(&mut vec![1, 2, 3, 4], 2.5);
    test_median(&mut vec![1, 2, 3], 2.0);
    test_median(&mut vec![1, 2, 2, 3], 2.0);
    test_median(&mut vec![4, 2, 1, 3], 2.5);

    // Mode
    test_mode(&vec![], &vec![]);
    test_mode(&vec![1], &vec![1]);
    test_mode(&vec![1, 2, 2, 4], &vec![2]);
    test_mode(&vec![1, 2, 3, 4], &vec![1, 2, 3, 4]);
    test_mode(&vec![1, 2, 2, 4, 3, 3], &vec![2, 3]);
}
