use indexmap::IndexMap;
use itertools::Itertools;
use rayon::prelude::*;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn read_seeds(line: String) -> (Vec<i64>, Vec<(i64, i64)>) {
    let full_line: Vec<&str> = line.split(": ").collect();
    let numbers: &str = full_line[1];

    let seeds_str: Vec<&str> = numbers.split_whitespace().collect();
    let seeds_part1: Vec<i64> = seeds_str
        .into_iter()
        .map(|s| s.parse::<i64>())
        .filter_map(Result::ok)
        .collect();
    let seeds_part2: Vec<(i64, i64)> = make_seeds_part2(&seeds_part1);

    (seeds_part1, seeds_part2)
}

fn make_seeds_part2(seeds: &Vec<i64>) -> Vec<(i64, i64)> {
    let real_seeds: Vec<(i64, i64)> = seeds
        .chunks(2)
        .map(|chunk| {
            let mut iter = chunk.iter();
            (
                iter.next().unwrap_or(&0).clone(),
                iter.next().unwrap_or(&0).clone(),
            )
        })
        .collect();

    let real_seeds_sorted: Vec<(i64, i64)> = real_seeds
        .into_iter()
        .sorted_by_key(|&(a, b)| (a, b))
        .collect();
    println!("{:?}", real_seeds_sorted);
    real_seeds_sorted
}

fn compute_location(start_len: (i64, i64), almanac: &IndexMap<String, Vec<Vec<i64>>>) -> Vec<i64> {
    let mut next_value: i64 = 0;
    let mut final_values: Vec<i64> = Vec::new();
    let start = start_len.0;
    let len = start_len.1;
    println!("{}, {}", start, len);
    for i in start..(len + start) {
        if next_value == 0 {
            next_value = i;
        }
        for key in almanac.keys() {
            for j in &almanac[key] {
                if next_value >= j[1] && next_value < j[1] + j[2] {
                    next_value = next_value - j[1] + j[0];
                    break;
                }
            }
        }
        final_values.push(next_value);
        next_value = 0;
    }

    final_values
}

fn part1(seeds_part1: Vec<i64>, almanac: &IndexMap<String, Vec<Vec<i64>>>) -> Vec<i64> {
    let mut next_value: i64 = 0;
    let mut final_values: Vec<i64> = Vec::new();

    for i in seeds_part1 {
        if next_value == 0 {
            next_value = i;
        }
        for key in almanac.keys() {
            for j in &almanac[key] {
                //if next_value >= j[1] && next_value < j[1] + j[2] {
                if next_value >= j[0] && next_value < j[1] {
                    //next_value = next_value - j[1] + j[0];
                    next_value += j[2];
                    break;
                }
            }
        }
        final_values.push(next_value);
        next_value = 0;
    }

    final_values
}

fn part2(seeds_part2: Vec<(i64, i64)>, almanac: &IndexMap<String, Vec<Vec<i64>>>) -> Vec<i64> {
    // Parallelize through seed ranges as tuples
    let locations: Vec<Vec<i64>> = seeds_part2
        .par_iter()
        .map(|&(a, b)| compute_location((a, b), &almanac))
        .collect();

    // Flatten the nested vector to get all location values and find the minimum
    let locations_flattened: Vec<i64> = locations
        .into_iter()
        .flat_map(|inner_vec| inner_vec)
        .collect();

    locations_flattened
}

fn optimization(mapping: Vec<Vec<(i64, i64, i64)>>, seed_range: (i64, i64)) -> Vec<(i64, i64)> {
    // Original input interval
    //let seed_range = Interval::new_inclusive(seed.0, seed.1);

    // Vector to store the output intervals
    let mut output_intervals: Vec<(i64, i64)> = Vec::new();

    let mut mapping_dispose: Vec<Vec<(i64, i64, i64)>> = mapping.clone();

    // Iterate until there are no more remaining intervals
    for mut mapping in mapping_dispose {
        while !mapping.is_empty() {
            // Take the first interval from the remaining intervals
            let (start, end, processing_value) = mapping.pop().unwrap();
            //println!("{}, {}, {}", start, end, processing_value);

            let common_start = seed_range.0.max(start);
            let common_end = seed_range.1.min(end);
            println!("{}", common_start);

            if common_start <= common_end {
                // There is an intersection
                // Modify the start and end values after checking for common intervals
                let modified_start = start + processing_value;
                let modified_end = end + processing_value;

                let remaining_interval_input = (common_end + 1, seed_range.1);
                let remaining_interval_multiple = (common_end + 1, modified_end, processing_value);

                // Store the output intervals in the vector as tuples
                /*output_intervals.push((
                    (modified_start, modified_end, processing_value),
                    (common_start, common_end),
                    (remaining_interval_input.0, remaining_interval_input.1),
                    (remaining_interval_multiple.0, remaining_interval_multiple.1),
                ));*/
                output_intervals.push((modified_start, modified_end));
                println!("{:?}", output_intervals);

                // Update the remaining intervals vector with the new remaining interval
                mapping.push(remaining_interval_multiple);
            }
        }
    }

    output_intervals
}

fn main() {
    let input_file = Path::new("./input.txt");

    let mut head: String = String::new();
    //let mut almanac: IndexMap<String, Vec<Vec<i64>>> = IndexMap::new();
    let mut almanac: Vec<Vec<(i64, i64, i64)>> = Vec::new();

    let mut current_vector: Vec<(i64, i64, i64)> = Vec::new();

    let mut seeds_part1: Vec<i64> = Vec::new();
    let mut seeds_part2: Vec<(i64, i64)> = Vec::new();

    if let Ok(lines) = read_lines(input_file) {
        for line in lines {
            if let Ok(ip) = line {
                if ip.trim().is_empty() {
                    if !current_vector.is_empty() {
                        //almanac.insert(head.clone(), current_vector);
                        almanac.push(current_vector);
                        current_vector = Vec::new();
                    }
                } else {
                    if ip.contains("seeds: ") {
                        (seeds_part1, seeds_part2) = read_seeds(ip);
                        continue;
                    }
                    if ip.contains("map") {
                        let full_line: Vec<&str> = ip.split(" map:").collect();
                        head = full_line[0].to_string();
                        continue;
                    }
                    let numbers_str: Vec<&str> = ip.split_whitespace().collect();
                    let numbers_vec2: Vec<i64> = numbers_str
                        .into_iter()
                        .map(|s| s.parse::<i64>())
                        .filter_map(Result::ok)
                        .collect();
                    let translation: (i64, i64, i64) = (
                        numbers_vec2[1],
                        numbers_vec2[1] + numbers_vec2[2],
                        numbers_vec2[0] - numbers_vec2[1],
                    );
                    current_vector.push(translation);
                }
            }
        }

        //almanac.insert(head.clone(), current_vector);
        almanac.push(current_vector);
    }

    //let locations_part1: Vec<i64> = part1(seeds_part1, &almanac);

    /*if let Some(min_value) = locations_part1.iter().cloned().min() {
        println!("The lowest number for part 1 is: {:?}", min_value);
    } else {
        println!("The vector is empty.");
    }*/

    //let locations_part2: Vec<i64> = part2(seeds_part2, &almanac);

    let mut result: Vec<Vec<(i64, i64)>> = Vec::new();
    for i in seeds_part2 {
        result.push(optimization(almanac.clone(), i))
    }

    if let Some(min_value) = result.iter().cloned().min() {
        println!("The lowest number for part 2 is: {:?}", min_value);
    } else {
        println!("The vector is empty.");
    }

    let flattened_tuples: Vec<(i64, i64)> = result.into_iter().flatten().collect();

    // Use the min function to find the minimum tuple based on the first element
    if let Some(min_tuple) = flattened_tuples.iter().min_by_key(|&(start, _)| start) {
        // Process or print the minimum tuple
        println!("Minimum Tuple: {:?}", min_tuple);
    } else {
        // Handle the case when the vector is empty
        println!("Vector of vectors is empty");
    }
}
