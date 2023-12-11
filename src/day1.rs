use std::collections::HashMap;
use std::fs::{self};
use std::str;

pub fn run() {
    let input = fs::read_to_string("./day1.input");
    let binding = input.unwrap();
    let lines: Vec<&str> = binding.lines().collect();
    let mut total: i32 = 0;
    let numbers_strings = [
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let numbers = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];
    let mut string_number_map = HashMap::new();
    string_number_map.insert("zero", 0);
    string_number_map.insert("one", 1);
    string_number_map.insert("two", 2);
    string_number_map.insert("three", 3);
    string_number_map.insert("four", 4);
    string_number_map.insert("five", 5);
    string_number_map.insert("six", 6);
    string_number_map.insert("seven", 7);
    string_number_map.insert("eight", 8);
    string_number_map.insert("nine", 9);

    for line in &lines {
        let mut result = vec![-1; 100];
        for number_string in numbers_strings {
            let indices: Vec<_> = line.match_indices(number_string).collect();
            for indice in indices {
                let (index, number) = indice;
                let num = string_number_map.get(&number).copied().unwrap();
                result[index] = num;
            }
        }
        for number in numbers {
            let indices: Vec<_> = line.match_indices(number).collect();
            for indice in indices {
                let (index, number) = indice;
                let num: i32 = number.parse().unwrap();
                result[index] = num;
            }
        }

        let filtered: Vec<i32> = result.into_iter().filter(|x| x.is_positive()).collect();
        let first = filtered.first().unwrap().to_string();
        let last = filtered.last().unwrap().to_string();

        let sum: i32;
        sum = (first + &last).to_owned().parse().unwrap();
        total += sum;
    }
    println!("Total sum: {:?}", total);
}
