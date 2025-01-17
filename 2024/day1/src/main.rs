use std::fs;
use std::path::Path;

fn part1(vec1: &mut Vec<i32>, vec2: &mut Vec<i32>) -> i32 {
    vec1.sort();
    vec2.sort();
    //println!("{:?}", vec2);
    let result: i32 = vec1
        .iter()
        .zip(vec2.iter())
        .map(|(a, b)| (a - b).abs())
        .sum();
    result
}

fn part2(vec1: &Vec<i32>, vec2: &Vec<i32>) -> i32 {
    let mut total_count: i32 = 0;
    for i in vec1.iter() {
        let count = vec2.iter().filter(|x| *x == i).count();
        total_count += i * count as i32;
    }
    total_count
}

fn parse_to_vec(file_string: String) -> (Vec<i32>, Vec<i32>) {
    let mut vec1: Vec<i32> = vec![];
    let mut vec2: Vec<i32> = vec![];
    for line in file_string.lines() {
        let fields: Vec<&str> = line.split_whitespace().collect();
        let field1 = fields[0].parse::<i32>().expect("Could not parse as i32");
        let field2 = fields[1].parse::<i32>().expect("Could not parse as i32");
        vec1.push(field1);
        vec2.push(field2);
    }
    (vec1, vec2)
}

fn main() {
    let input = Path::new("./input.txt");
    let file_content = fs::read_to_string(input).expect("Error reading file content");

    let (mut vec1, mut vec2) = parse_to_vec(file_content);

    let part1_result = part1(&mut vec1, &mut vec2);
    println!("{}", part1_result);
    let part2_result = part2(&vec1, &vec2);
    println!("{}", part2_result);
}
