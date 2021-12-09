use std::fs;

pub fn main() -> usize {
    let counts = vec![2, 3, 4, 7];

    let raw = fs::read_to_string("src/day08/input.txt").expect("Unable to read file");
    let rows: Vec<&str> = raw.split("\n").collect();
    let split: Vec<Vec<Vec<&str>>> = rows
        .into_iter()
        .map(|s| {
            s.split(" | ")
                .map(|s| s.split(" ").collect::<Vec<&str>>())
                .collect()
        })
        .collect();

    let count_part_1 = split.into_iter().fold(0, |acc, line| {
        acc + line
            .last()
            .unwrap()
            .into_iter()
            .filter(|s| counts.contains(&s.len()))
            .count()
    });

    count_part_1
}
