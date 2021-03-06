//  Created by Eduardo Flores on 5/18/20.
//  edfloreshz@gmail.com

// Given a list of integers, use a vector and return the mean (the average value),
// median (when sorted, the value in the middle position), and mode (the value that
// occurs most often; a hash map will be helpful here) of the list.

use std::collections::HashMap;

pub fn test_grades() {
    let mut grades = [
        10, 10, 10, 9, 9, 8, 8, 8, 8, 8, 7, 7, 10, 9, 6, 7, 8, 8, 9, 10,
    ];
    let mean = mean(&grades);
    let median = median(&mut grades);
    let mode = mode(&grades);
    println!("Mean: {} \nMedian: {} \nMode: {}", mean, median, mode);
}

pub fn mean(grades: &[i32]) -> i32 {
    grades.iter().sum::<i32>() / grades.len() as i32
}

pub fn median(grades: &mut [i32]) -> i32 {
    grades.sort();
    if grades.len() % 2 == 0 {
        let middle = grades.len() / 2;
        (grades[middle] + grades[middle + 1]) / 2
    } else { grades[grades.len() / 2] }
}

pub fn mode(grades: &[i32]) -> i32 {
    let mut hashed_grades = HashMap::new();
    let mut mode = (0, 0);
    let mut max = 0;
    for grade in grades {
        *hashed_grades.entry(*grade).or_insert(0) += 1;
    }
    for (key, value) in hashed_grades {
        if value > max {
            max = value;
            mode = (key, value);
        }
    }
    mode.0
}
