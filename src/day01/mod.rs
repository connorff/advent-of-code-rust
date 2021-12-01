use std::fs;

pub fn submarine() -> usize {
    let data = fs::read_to_string("src/day01/nums.txt").expect("Unable to read file");

    let strings = data.split('\n');
    let nums: Vec<_> = strings.map(|num| num.parse::<i32>().unwrap()).collect();

    let increase_bools = nums.windows(2).map(|w| w[1] > w[0]);
    let increases = increase_bools.filter(|b| *b).count();

    return increases;
}
