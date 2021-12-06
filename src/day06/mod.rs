use std::{collections::HashMap, fs};

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

    fn descendants_after_days(&self, days: i32) -> Vec<LanternFish> {
        let mut fish_vec = vec![self.clone()];

        for _ in 0..days {
            for x in 0..fish_vec.len() {
                let fish = &mut fish_vec[x];

                let mut new_fish = fish.add_day();
                fish_vec.append(&mut new_fish);
            }
        }

        fish_vec
    }
}

pub fn main() -> i64 {
    let data = fs::read_to_string("src/day06/fish.txt").expect("Unable to read file");
    let initial_fish: Vec<LanternFish> = data
        .split(",")
        .map(|num| num.parse::<i32>().unwrap())
        .map(|timer| LanternFish { timer })
        .collect();

    let fish_map_nums: Vec<i32> = (0..9).collect();
    let fish_num_map: HashMap<_, _> = fish_map_nums
        .into_iter()
        .map(|n| LanternFish { timer: n })
        .map(|fish| (fish.timer, fish.descendants_after_days(128).len() as i64))
        .collect();

    let fish_vec: Vec<LanternFish> = initial_fish
        .into_iter()
        .map(|fish| fish.descendants_after_days(128))
        .flatten()
        .collect();

    fish_vec
        .into_iter()
        .map(|f| *fish_num_map.get(&f.timer).unwrap())
        .sum()
}
