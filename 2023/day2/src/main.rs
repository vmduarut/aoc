use std::fs;
use std::path::Path;

fn is_possible(line: String) -> bool {
    let games: Vec<&str> = line.split(": ").collect();
    let mut possible = true;

    let last_element = games.last().unwrap_or(&"No elements found");

    let sets: Vec<&str> = last_element.split("; ").collect();

    //println!("{}", last_element);
    'outer: for i in sets {
        //println!("{}, set: {}", games[0], i);
        let boxes: Vec<&str> = i.split(", ").collect();
        for j in boxes {
            //println!("{}", j);
            let colors: Vec<&str> = j.split(" ").collect();
            let color: &str = colors[1];
            let number: i32 = colors[0].parse::<i32>().unwrap();
            if color == "red" && number > 12 {
                possible = false;
                break 'outer;
            }
            if color == "green" && number > 13 {
                possible = false;
                break 'outer;
            }
            if color == "blue" && number > 14 {
                possible = false;
                break 'outer;
            }
        }
    }
    possible
}

fn main() {
    let input_file = Path::new("./input.txt");
    let file_content = fs::read_to_string(input_file).expect("Error reading file content");

    let mut sum_games = 0;

    for line in file_content.lines() {
        let games: Vec<&str> = line.split(": ").collect();
        let game_number_vec: Vec<&str> = games[0].split(" ").collect();
        let game_number = game_number_vec[1].parse::<i32>().unwrap();
        //is_possible(line.to_string());
        if is_possible(line.to_string()) {
            println!("yes, {}", game_number);
            sum_games += game_number;
        }
    }
    println!("{:?}", sum_games);
}
