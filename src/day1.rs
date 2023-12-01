use std::fs::{self};

pub fn run() {
    let input = fs::read_to_string("./day1.input");
    let binding = input.unwrap();
    let lines: Vec<&str> = binding.lines().collect();
    let mut total = 0;
    for line in &lines {
        let chars_numbers: Vec<char> = line
            .chars()
            .filter(|c| c.is_numeric())
            .into_iter()
            .collect();
        let first = chars_numbers.first().unwrap().to_string();
        let last = chars_numbers.last().unwrap().to_string();
        let number_string: String = first + &last;
        total = total + number_string.parse::<u32>().unwrap();
    }
    println!("{:?}", total);
}
