use std::{fs};

#[derive(Copy, Clone)]
struct LanternFish {
    timer: i32,
}

impl LanternFish {
    fn add_day(&mut self) -> Vec<LanternFish> {
        self.timer -= 1;
        if self.timer == -1 {
            self.timer = 6;
            return vec![LanternFish { timer: 8 }];
        }

        return vec![];
    }
}

pub fn main() -> usize {
    let data = fs::read_to_string("src/day06/fish.txt").expect("Unable to read file");
    let initial_fish: Vec<i32> = data
        .split(",")
        .map(|num| num.parse::<i32>().unwrap())
        .collect();
    let mut fish_vec: Vec<LanternFish> = initial_fish
        .into_iter()
        .map(|timer| LanternFish { timer })
        .collect();

    for _ in 0..80 {
        for x in 0..fish_vec.len() {
            let fish = &mut fish_vec[x];

            let mut new_fish = fish.add_day();
            fish_vec.append(&mut new_fish);
        }
    }

    fish_vec.len()
}
