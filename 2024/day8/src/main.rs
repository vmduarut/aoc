use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;

fn in_bounds(pos: &(i64, i64), rows: &i64, cols: &i64) -> bool {
    pos.0 >= 1 && pos.0 <= *rows && pos.1 >= 1 && pos.1 <= *cols
}

fn main() {
    // Create a hashmap to hold all data from input, all chars different that '.' as keys, and as values each position in x,y as tuples
    // Then after, anothewr loop traverses through all combinations of tuple values for each char, it computes the distance between them
    // (como el caballo en ajedrez) and checks several conditions: if it is inside the map (not outside limits) and if one anntena is twice the distance from the other.
    // If these conditions are present, add the resulting to a final count.
    // Also, the result has to be another map, or a set, with the positions of the antinodes (resulting from before), so if a new antinode is already presernt in the same place,
    // it does not count as another.
    // With itertools combinations we can make combis with all pairs
    let input = Path::new("./input.txt");
    let file_content = fs::read_to_string(input).expect("Error reading file content");

    let mut initial_map: HashMap<char, Vec<(i64, i64)>> = HashMap::new();
    let mut counter: HashSet<(i64, i64)> = HashSet::new();
    let mut max_x: i64 = 0;
    let mut max_y: i64 = 0;
    for line in file_content.lines() {
        max_x += 1;
        if max_y == 0 {
            max_y = line.chars().count() as i64;
        }
        for (i, cha) in line.chars().enumerate() {
            if cha != '.' {
                //println!("{}", cha);
                initial_map
                    .entry(cha)
                    .or_default()
                    .push((max_x, i as i64 + 1));
            }
        }
        //println!("{}", line);
    }
    println!("{:?}", initial_map);
    println!("Max x, {}, max y {}", max_x, max_y);

    for cha in initial_map.values() {
        for i in cha.iter().combinations(2) {
            let node_1 = i[0];
            let node_2 = i[1];
            let distance = (node_2.0 - node_1.0, node_2.1 - node_1.1);
            let possible_node_1 = (node_1.0 - distance.0, node_1.1 - distance.1);
            let possible_node_2 = (node_2.0 + distance.0, node_2.1 + distance.1);
            if in_bounds(&possible_node_1, &max_x, &max_y) && !counter.contains(&possible_node_1) {
                counter.insert(possible_node_1);
            }
            if in_bounds(&possible_node_2, &max_x, &max_y) && !counter.contains(&possible_node_2) {
                counter.insert(possible_node_2);
            }
            println!("{:?}, {:?}", i[0], i[1]);
        }
    }
    println!("total is {:?}", counter.len());

    // part 2
    for cha in initial_map.values() {
        for i in cha.iter().combinations(2) {
            let &(mut node_1) = i[0];
            let &(mut node_2) = i[1];
            counter.insert(node_1);
            counter.insert(node_2);

            let distance = (node_2.0 - node_1.0, node_2.1 - node_1.1);
            loop {
                node_1 = (node_1.0 - distance.0, node_1.1 - distance.1);
                if !in_bounds(&node_1, &max_x, &max_y) {
                    break;
                }
                counter.insert(node_1);
            }
            loop {
                node_2 = (node_2.0 + distance.0, node_2.1 + distance.1);
                if !in_bounds(&node_2, &max_x, &max_y) {
                    break;
                }
                counter.insert(node_2);
            }
        }
    }
    println!("total is {:?}", counter.len());
}
