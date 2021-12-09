use std::fs;

fn get_dst(dst: i32) -> i32 {
    (dst * (dst + 1)) / 2
}

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

    let mut res = i32::MAX;
    for num in *crab_positions.first().unwrap()..*crab_positions.last().unwrap() {
        let curr_res = crab_positions
            .iter()
            .fold(0, |total, pos| total + get_dst((pos - num).abs()));
        if curr_res < res {
            res = curr_res;
        }
    }

    res
}
