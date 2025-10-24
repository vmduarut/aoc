use std::fs;
use std::path::Path;

fn main() {
    let input = Path::new("./input.txt");
    let file_content = fs::read_to_string(input).expect("Error reading file content");

    let mut counter: i32 = -1;

    let mut signal: Vec<_> = file_content
        .chars()
        .filter_map(|c| c.to_digit(10))
        .enumerate()
        .flat_map(|(i, c)| {
            let times = c as usize;
            let token = if i % 2 == 0 {
                counter += 1;
                counter.to_string()
            } else {
                ".".to_string()
            };
            std::iter::repeat(token).take(times)
        })
        .collect();
    println!("{:?}", signal);

    let mut i = 0;
    let mut j = signal.len() - 1;

    while i < j {
        if signal[i] == "." {
            if signal[j] == "." {
                j -= 1;
                continue;
            }
            signal.swap(i, j);
            i += 1;
            j -= 1;
        } else {
            i += 1;
        }
    }

    let result: i64 = signal
        .iter()
        .filter(|c| c.to_string() != ".")
        .enumerate()
        .map(|(i, e)| i as i64 * e.parse::<i64>().expect(""))
        .sum();

    println!("{:?}", result);
}
