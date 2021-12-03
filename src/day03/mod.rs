use std::fs;

pub fn gamma() -> Vec<bool> {
    let data = fs::read_to_string("src/day03/diagnostic.txt").expect("Unable to read file");
    let diagnostics: Vec<Vec<&str>> = data
        .split('\n')
        .map(|num| num.split("").filter(|s| *s != "").collect())
        .collect();

    let columns: Vec<Vec<i32>> = diagnostics
        .first()
        .unwrap()
        .iter()
        .enumerate()
        .map(|(index, _)| {
            diagnostics
                .iter()
                .map(|row| row.iter().nth(index).unwrap().parse::<i32>().unwrap())
                .collect()
        })
        .collect();

    columns
        .iter()
        .map(|col| col.iter().sum::<i32>() as usize > col.iter().len() / 2)
        .collect::<Vec<bool>>()
}

pub fn epsilon(gamma: Vec<bool>) -> Vec<bool> {
    gamma.iter().map(|b| !b).collect()
}

pub fn to_int(bools: Vec<bool>) -> isize {
    let bin_src = &bools
        .iter()
        .map(|b| (*b as i32).to_string())
        .collect::<Vec<String>>()
        .join("")
        .to_owned()[..];

    isize::from_str_radix(bin_src, 2).unwrap()
}
