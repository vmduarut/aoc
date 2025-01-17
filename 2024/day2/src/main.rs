use std::collections::HashSet;
use std::fs;
use std::path::Path;
fn part1(file_content: &String) -> i32 {
    let mut safe_reports: i32 = 0;
    for line in file_content.lines() {
        let levels: Vec<i32> = line
            .split_whitespace()
            .map(|a| a.parse::<i32>().expect("Could not parse as i32"))
            .collect();
        if is_safe(&levels) {
            safe_reports += 1;
        }
    }
    safe_reports
}

fn is_safe(levels: &Vec<i32>) -> bool {
    let possible_values_pos = HashSet::from([1, 2, 3]);
    let possible_values_neg = HashSet::from([-1, -2, -3]);
    let mut differences: Vec<i32> = vec![];
    for window in levels.windows(2) {
        let difference = window[0] - window[1];
        differences.push(difference);
    }
    let is_ascending = differences
        .iter()
        .all(|&x| possible_values_pos.contains(&x));
    let is_descending = differences
        .iter()
        .all(|&x| possible_values_neg.contains(&x));
    is_ascending || is_descending
}

fn part2(file_content: &String) -> i32 {
    let mut safe_reports: i32 = 0;
    for line in file_content.lines() {
        let levels: Vec<i32> = line
            .split_whitespace()
            .map(|a| a.parse::<i32>().expect("Could not parse as i32"))
            .collect();
        if is_safe(&levels) {
            safe_reports += 1;
        } else {
            for i in 0..levels.len() {
                let mut levels_cloned = levels.clone();
                levels_cloned.remove(i);
                if is_safe(&levels_cloned) {
                    safe_reports += 1;
                    break;
                }
            }
        }
    }
    safe_reports
}

fn main() {
    let input = Path::new("./input.txt");
    let file_content = fs::read_to_string(input).expect("Error reading file content");

    let result_part1 = part1(&file_content);
    let result_part2 = part2(&file_content);
    println!("{}", result_part1);
    println!("{}", result_part2);
}
