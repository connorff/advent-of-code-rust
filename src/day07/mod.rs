use std::fs;

pub fn main() -> i32 {
    let data = fs::read_to_string("src/day07/crabs.txt").expect("Unable to read file");
    let mut crab_positions: Vec<i32> = data
        .split(",")
        .map(|num| num.parse::<i32>().unwrap())
        .collect();
    crab_positions.sort();

    let mid = crab_positions.len() / 2;
    let median = if crab_positions.len() % 2 == 0 {
        (crab_positions[mid - 1] + crab_positions[mid]) / 2
    } else {
        crab_positions[mid - 1]
    };

    crab_positions.iter().fold(0, |total, pos| total + (pos - median).abs())
}