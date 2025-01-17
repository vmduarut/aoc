use regex::Regex;
use std::fs;
use std::path::Path;

fn part1(pattern: &Regex, file_content: &String) -> i32 {
    pattern
        .captures_iter(&file_content)
        .map(|capture| {
            let char1 = &capture[1].parse::<i32>().expect("Not possible i32");
            let char2 = &capture[2].parse::<i32>().expect("Not possible i32");
            char1 * char2
        })
        .sum()
}

fn part2(pattern: &Regex, file_content: &String) -> i32 {
    let mut dont: bool = false;
    pattern
        .captures_iter(&file_content)
        .filter_map(|capture| {
            if let Some(_number) = capture.get(1) {
                if !dont {
                    let char1 = &capture[1].parse::<i32>().expect("Not possible i32");
                    let char2 = &capture[2].parse::<i32>().expect("Not possible i32");
                    Some(char1 * char2)
                } else {
                    None
                }
            } else if let Some(dont_pattern) = capture.get(3) {
                if dont_pattern.as_str() == "don't()" {
                    dont = true;
                } else {
                    dont = false;
                }
                None
            } else {
                None
            }
        })
        .sum()
}
fn main() {
    let pattern_part1 = Regex::new(r"mul\((\d+),(\d+)\)").expect("No valid regex");
    let input = Path::new("./input.txt");
    let file_content = fs::read_to_string(input).expect("Error reading file content");
    let result_part1 = part1(&pattern_part1, &file_content);
    println!("{}", result_part1);

    let pattern_part2 = Regex::new(r"mul\((\d+),(\d+)\)|(do(n't)?\(\))").expect("No valid regex");
    let result_part2 = part2(&pattern_part2, &file_content);
    println!("{}", result_part2);
}
