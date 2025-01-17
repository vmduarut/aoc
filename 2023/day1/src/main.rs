use std::fs;
use std::path::Path;
use hashbrown::HashMap;

fn first_last(text: String, numbers: &HashMap<&str, &str>) -> String {
    let mut first: Option<String> = None;
    let mut last: Option<String> = None;

    let mut characters: Vec<char> = Vec::new();
    let mut previous_num: String = String::new();

    for char in text.chars() {
        if char.is_numeric() {
            if first.is_none() {
                first = Some(char.to_string());
            }
            last = Some(char.to_string());
            characters.clear();
        }
        else {
            characters.push(char);
            let mut result: String = characters.iter().collect();
            
            let is_equal = numbers.keys().any(|key| key.chars().skip(1).collect::<String>() == result);

            if is_equal {
                if numbers.contains_key(previous_num.as_str()) {
                    let last_char: String = previous_num.chars().last().unwrap().to_string();
                    result = last_char + &result;
                }
            }
            for key in numbers.keys() {
                if result.contains(key) {
                    previous_num = key.to_string();
                    if first.is_none() {
                        first = numbers.get(key).map(|v| v.to_string());
                    }
                    last = numbers.get(key).map(|v| v.to_string());
                    characters.clear();
                    break;
                }
            }
        }
    }

    let mut first_last_num = String::new();
    if let Some(num) = first {
        first_last_num.push_str(&num);
    }

    if let Some(num) = last {
        first_last_num.push_str(&num);
    }

    println!("{}, {}", first_last_num, text);

    first_last_num
}

fn main() {
    let mut numbers: HashMap<&str, &str> = HashMap::new();

    numbers.insert("one", "1");
    numbers.insert("two", "2");
    numbers.insert("three", "3");
    numbers.insert("four", "4");
    numbers.insert("five", "5");
    numbers.insert("six", "6");
    numbers.insert("seven", "7");
    numbers.insert("eight", "8");
    numbers.insert("nine", "9");

    let text_file = Path::new("./input_day1.txt");
    let file_content = fs::read_to_string(text_file).expect("Error reading file content");

    let mut calibration_values = 0;
    for line in file_content.lines() {
        calibration_values += first_last(line.to_string(), &numbers).parse::<i32>().unwrap();
    }
    println!("{:?}", calibration_values);
}
