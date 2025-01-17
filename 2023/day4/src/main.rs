use std::collections::BTreeMap;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn process_line(line: String) -> (Vec<i32>, Vec<i32>) {
    //Process line from file to get a tuple with winning numbers and our numbers on vectors
    let full_line: Vec<&str> = line.split(": ").collect();
    let numbers: &str = full_line[1];

    let numbers_vec: Vec<&str> = numbers.split(" | ").collect();
    let winning: Vec<&str> = numbers_vec[0].split_whitespace().collect();
    let winning_numbers: Vec<i32> = winning
        .into_iter()
        .map(|s| s.parse::<i32>())
        .filter_map(Result::ok)
        .collect();

    let ours: Vec<&str> = numbers_vec[1].split_whitespace().collect();
    let ours_numbers: Vec<i32> = ours
        .into_iter()
        .map(|s| s.parse::<i32>())
        .filter_map(Result::ok)
        .collect();

    (winning_numbers, ours_numbers)
}

fn match_count_part1(winning: &Vec<i32>, ours: &Vec<i32>) -> i32 {
    let mut count: i32 = 0;
    for i in ours {
        if winning.contains(&i) {
            if count == 0 {
                count = count + 1
            } else {
                count = count * 2
            }
        }
    }

    count
}

fn match_count_part2(winning: &Vec<i32>, ours: &Vec<i32>) -> i32 {
    let mut count: i32 = 0;
    for i in ours {
        if winning.contains(&i) {
            count = count + 1
        }
    }

    count
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    let input_file = Path::new("./input.txt");
    //let file_content = fs::read_to_string(input_file).expect("Error reading file content");

    let mut result_part1: i32 = 0;

    let mut cards_copies: HashMap<i32, i32> = HashMap::new();
    let mut cards_numbers: BTreeMap<i32, (Vec<i32>, Vec<i32>)> = BTreeMap::new();

    let mut line_number: i32 = 1;

    // Opening file to get BTreeMap with card number and tuples
    if let Ok(lines) = read_lines(input_file) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                cards_numbers.insert(line_number, process_line(ip.to_string()));
                cards_copies.insert(line_number, 1);

                line_number += 1;
            }
        }
    }

    // Part 1
    for numbers_tuples in cards_numbers.values() {
        result_part1 += match_count_part1(&numbers_tuples.0, &numbers_tuples.1);
    }

    println!("Sum for part 1: {}", result_part1);

    // Part 2
    for (&key, value) in &cards_numbers {
        let len = key + match_count_part2(&value.0, &value.1);
        let actual_count = cards_copies[&key];
        // Loop from actual card to next based on how many winning numbers has the card, then add the sum of original and copies card for each
        for i in key + 1..=len {
            if let Some(entry) = cards_copies.get_mut(&i) {
                *entry += actual_count;
            }
        }
    }

    let result_part2: i32 = cards_copies.values().cloned().sum();

    println!("Sum for part 2 is: {}", result_part2);
}
