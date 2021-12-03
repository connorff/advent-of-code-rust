use std::fs;
mod day03;

fn main() {
    let raw = fs::read_to_string("src/day03/diagnostic.txt").expect("Unable to read file");
    let data: Vec<&str> = raw.split("\n").collect();

    let gamma_vec = day03::gamma(data.clone());
    let epsilon = day03::epsilon(data.clone());
    println!("{}", day03::to_int(gamma_vec) * epsilon);

    println!(
        "{:?}",
        day03::oxygen(data.clone()) * day03::co2(data.clone())
    );
}
