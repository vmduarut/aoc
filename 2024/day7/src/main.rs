use std::fs;
use std::path::Path;

fn process_line(current: &i64, numbers: &Vec<i64>, result: &i64, index: usize) -> bool {
    if current > result {
        return false;
    }
    if current == result {
        return true;
    }
    if index == numbers.len() {
        return false;
    }
    return (process_line(&(current + numbers[index]), numbers, result, index + 1))
        || (process_line(&(current * numbers[index]), numbers, result, index + 1))
        || (process_line(
            &(format!("{}{}", current, numbers[index]).parse().unwrap()),
            numbers,
            result,
            index + 1,
        ));
}

fn main() {
    let input = Path::new("./input.txt");
    let file_content = fs::read_to_string(input).expect("Error reading file content");
    //let matrix: Vec<_> = file_content
    //    .lines()
    //    .inspect(|line| println!("{}", line))
    //    .collect();
    //.map(|line| line.trim().chars().collect::<Vec<_>>())
    //.collect();

    let mut total = 0;

    for line in file_content.lines() {
        // TODO: Process each component inside match
        let (result, numbers) = match line.split_once(':') {
            Some((result, numbers)) => {
                let result: i64 = result.trim().parse().expect("a is not a valid i32");
                let numbers: Vec<i64> = numbers
                    .split_whitespace()
                    .map(|number| number.trim().parse().expect("is not valid i64"))
                    .collect();
                (result, numbers)
            }
            _ => panic!("Problem"),
        };
        //println!("{}, {:?}", result, numbers);

        let good_line = process_line(&numbers[0], &numbers, &result, 1);
        if good_line == true {
            println!("Good line!");
            println!("{}: {:?}", result, numbers);
            total = total + result;
        }
    }

    println!("Total is: {}", total);
}
