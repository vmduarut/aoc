use std::collections::HashSet;
use std::fs;
use std::path::Path;

fn main() {
    let input = Path::new("./input.txt");
    let file_content = fs::read_to_string(input).expect("Error reading file content");
    let matrix: Vec<_> = file_content
        .lines()
        .map(|line| line.trim().chars().collect::<Vec<_>>())
        .collect();
    let mut initial_guard = None;
    for (i, row) in matrix.iter().enumerate() {
        for (j, &ch) in row.iter().enumerate() {
            if ch == '^' {
                initial_guard = Some((i, j));
                break;
            }
        }
    }
    let mut distinct_positions = HashSet::new();
    let mut direction = "up";
    println!("{:?}", initial_guard);
    if let Some((mut i, mut j)) = initial_guard {
        loop {
            if i == matrix.len() || j == matrix[i].len() {
                println!("Out of bounds!");
                break;
            }

            match direction {
                "up" => {
                    if matrix[i][j] == '#' {
                        direction = "right";
                        i += 1;
                        continue;
                    } else {
                        distinct_positions.insert((i, j));
                        if i == 0 {
                            break;
                        }
                        i -= 1;
                    }
                }
                "right" => {
                    if matrix[i][j] == '#' {
                        direction = "down";
                        j -= 1;
                        continue;
                    } else {
                        distinct_positions.insert((i, j));
                        j += 1;
                    }
                }
                "down" => {
                    if matrix[i][j] == '#' {
                        direction = "left";
                        i -= 1;
                        continue;
                    } else {
                        distinct_positions.insert((i, j));
                        i += 1;
                    }
                }
                "left" => {
                    if matrix[i][j] == '#' {
                        direction = "up";
                        j += 1;
                        continue;
                    } else {
                        distinct_positions.insert((i, j));
                        if j == 0 {
                            break;
                        }
                        j -= 1;
                    }
                }
                _ => {
                    println!("Nothing");
                    break;
                }
            }
        }
    }
    println!("{}", distinct_positions.len());
}
