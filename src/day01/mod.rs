use std::fs;

pub fn load_nums() -> Vec<i32> {
    let data = fs::read_to_string("src/day01/nums.txt").expect("Unable to read file");

    data.split('\n')
        .map(|num| num.parse::<i32>().unwrap())
        .collect()
}

pub fn num_increases(nums: Vec<i32>) -> usize {
    nums.windows(2).filter(|w| w[1] > w[0]).count()
}

pub fn submarine() -> usize {
    num_increases(load_nums())
}

pub fn window_submarine() -> usize {
    let windows: Vec<i32> = load_nums().windows(3).map(|w| w.iter().sum()).collect();

    num_increases(windows)
}
