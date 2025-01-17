use regex::Regex;
use std::fs;
use std::path::Path;

fn main() {
    let input = Path::new("./input.txt");
    let file_content = fs::read_to_string(input).expect("Error reading file content");
    let pattern_part1 = Regex::new(r"XMAS").expect("No valid regex");
    let mut total_xmas = 0;
    let puzzle: Vec<_> = file_content
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect();
    // Go through columns
    // This part 1 can be handled like part 2
    for i in 0..puzzle[0].len() {
        let mut column: Vec<_> = vec![];
        for j in 0..puzzle.len() {
            column.push(puzzle[j][i]);
        }
        let column_string: String = column.clone().into_iter().collect();
        let column_rev: String = column.clone().into_iter().rev().collect();
        for xmas in pattern_part1.find_iter(&column_string) {
            total_xmas += 1;
        }
        for xmas in pattern_part1.find_iter(&column_rev) {
            total_xmas += 1;
        }
    }
    // Go through rows
    for i in puzzle.iter() {
        let row: String = i.clone().into_iter().collect();
        let row_rev: String = i.clone().into_iter().rev().collect();
        for xmas in pattern_part1.find_iter(&row) {
            total_xmas += 1;
        }
        for xmas in pattern_part1.find_iter(&row_rev) {
            total_xmas += 1;
        }
    }
    // Go through diagonal (top-left bottom-right)
    let total_diagonals = puzzle.len() + (puzzle[0].len() - 1);
    for i in 0..total_diagonals {
        let mut diagonal: Vec<_> = vec![];
        if i <= puzzle.len() {
            for row in (0..=i).rev() {
                let col = i - row;
                if row < puzzle.len() && col < puzzle[0].len() {
                    diagonal.push(puzzle[row][col]);
                }
            }
            // Process here diagonal
            let diagonal_string: String = diagonal.clone().into_iter().collect();
            let diagonal_rev: String = diagonal.clone().into_iter().rev().collect();
            for xmas in pattern_part1.find_iter(&diagonal_string) {
                total_xmas += 1;
            }
            for xmas in pattern_part1.find_iter(&diagonal_rev) {
                total_xmas += 1;
            }
        } else {
            for row in (i - puzzle.len()..puzzle.len()).rev() {
                let col = i - row;
                if col < puzzle[0].len() {
                    diagonal.push(puzzle[row][col]);
                }
            }
            // Process here diagonal
            let diagonal_string: String = diagonal.clone().into_iter().collect();
            let diagonal_rev: String = diagonal.clone().into_iter().rev().collect();
            for xmas in pattern_part1.find_iter(&diagonal_string) {
                total_xmas += 1;
            }
            for xmas in pattern_part1.find_iter(&diagonal_rev) {
                total_xmas += 1;
            }
        }
    }
    for i in 0..total_diagonals {
        let mut diagonal: Vec<_> = vec![];

        // If we are in the upper part of the diagonals (including the main diagonal)
        if i <= puzzle[0].len() {
            for row in (0..=i).rev() {
                let col = puzzle[0].len() as i32 - i as i32 + row as i32; // Calculate the column
                if row < puzzle.len() && col < puzzle[0].len() as i32 {
                    diagonal.push(puzzle[row][col as usize]);
                }
            }
            // Process here diagonal
            let diagonal_string: String = diagonal.clone().into_iter().collect();
            let diagonal_rev: String = diagonal.clone().into_iter().rev().collect();
            for xmas in pattern_part1.find_iter(&diagonal_string) {
                total_xmas += 1;
            }
            for xmas in pattern_part1.find_iter(&diagonal_rev) {
                total_xmas += 1;
            }
        } else {
            // On the lower part of the diagonals
            for row in (i - puzzle.len()..puzzle.len()).rev() {
                //let col = i - puzzle.len();
                let col = (puzzle.len() as i32 - i as i32) + row as i32; // Calculate the column
                if row < puzzle.len() && col >= 0 {
                    diagonal.push(puzzle[row][col as usize]);
                }
            }
            // Process here diagonal
            let diagonal_string: String = diagonal.clone().into_iter().collect();
            let diagonal_rev: String = diagonal.clone().into_iter().rev().collect();
            for xmas in pattern_part1.find_iter(&diagonal_string) {
                total_xmas += 1;
            }
            for xmas in pattern_part1.find_iter(&diagonal_rev) {
                total_xmas += 1;
            }
        }
    }
    println!("{}", total_xmas);
    let mut part2 = 0;
    let puzzle: Vec<_> = file_content
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect();
    // part 2
    for i in 1..puzzle.len() - 1 {
        for j in 1..puzzle[0].len() - 1 {
            if puzzle[i][j] == 'A' {
                if (puzzle[i - 1][j - 1] == 'M' && puzzle[i + 1][j + 1] == 'S')
                    || (puzzle[i - 1][j - 1] == 'S' && puzzle[i + 1][j + 1] == 'M')
                {
                    if (puzzle[i - 1][j + 1] == 'M' && puzzle[i + 1][j - 1] == 'S')
                        || (puzzle[i - 1][j + 1] == 'S' && puzzle[i + 1][j - 1] == 'M')
                    {
                        part2 += 1;
                    }
                }
            }
        }
    }
    println!("{}", part2);
}
