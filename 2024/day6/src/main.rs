use std::collections::HashSet;
use std::fs;
use std::path::Path;

fn main() {
    let input = Path::new("./input.txt");
    let file_content = fs::read_to_string(input).expect("Error reading file content");
    let mut matrix: Vec<_> = file_content
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
    println!("Distinct positions: {}", distinct_positions.len());

    // Part 2
    let mut positions_loop = HashSet::new();
    for (x, y) in distinct_positions {
        let original = matrix[x][y];
        matrix[x][y] = '#';
        if is_loop(&matrix, initial_guard) {
            positions_loop.insert((x, y));
        }
        matrix[x][y] = original;
    }

    println!("Total obstructs: {:?}", positions_loop.len());
}

fn is_loop(matrix: &Vec<Vec<char>>, initial_guard: Option<(usize, usize)>) -> bool {
    let mut direction = "up";
    let mut visited: HashSet<(usize, usize, String)> = HashSet::new();
    if let Some((mut i, mut j)) = initial_guard {
        loop {
            if visited.contains(&(i, j, direction.to_string())) {
                return true;
            }
            if (i, j) != initial_guard.expect("No") {
                visited.insert((i, j, direction.to_string()));
            }
            if i == matrix.len() || j == matrix[i].len() {
                return false;
            }

            match direction {
                "up" => {
                    if matrix[i][j] == '#' {
                        direction = "right";
                        i += 1;
                    } else {
                        if i == 0 {
                            return false;
                        }
                        i -= 1;
                    }
                }
                "right" => {
                    if matrix[i][j] == '#' {
                        direction = "down";
                        j -= 1;
                    } else {
                        j += 1;
                    }
                }
                "down" => {
                    if matrix[i][j] == '#' {
                        direction = "left";
                        i -= 1;
                    } else {
                        i += 1;
                    }
                }
                "left" => {
                    if matrix[i][j] == '#' {
                        direction = "up";
                        j += 1;
                    } else {
                        if j == 0 {
                            return false;
                        }
                        j -= 1;
                    }
                }
                _ => {
                    println!("Nothing");
                    return false;
                }
            }
        }
    } else {
        return false;
    }
}
