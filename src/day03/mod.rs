fn get_rows(nums: Vec<&str>) -> Vec<Vec<&str>> {
    nums.iter()
        .map(|num| num.split("").filter(|s| *s != "").collect())
        .collect()
}

fn get_columns(str_rows: Vec<Vec<&str>>) -> Vec<Vec<i32>> {
    str_rows
        .first()
        .unwrap()
        .iter()
        .enumerate()
        .map(|(index, _)| {
            str_rows
                .clone()
                .iter()
                .map(|row| row.iter().nth(index).unwrap().parse::<i32>().unwrap())
                .collect()
        })
        .collect()
}

fn reduce_bins<'a>(bins: Vec<&'a str>, cmp: &dyn Fn(char, char) -> bool) -> i32 {
    let mut clones: Vec<&'a str> = bins.clone();

    for col in 0..clones[0].len() {
        let split_rows: Vec<Vec<&str>> = get_rows(clones.clone());
        let split_columns = get_columns(split_rows);
        let bit = most_common_bit(&split_columns[col]);

        clones = clones
            .into_iter()
            .filter(|row| {
                cmp(
                    row.chars().nth(col).unwrap(),
                    char::from_digit(bit as u32, 2).unwrap(),
                )
            })
            .collect();

        if clones.len() == 1 {
            break;
        }
    }

    i32::from_str_radix(clones.first().unwrap(), 2).unwrap()
}

fn most_common_bit(col: &Vec<i32>) -> i32 {
    let bit_sum = col.iter().sum::<i32>() as f32;
    let num_bits = col.iter().len() as f32;

    (bit_sum >= num_bits / 2.0) as i32
}

pub fn gamma(data: Vec<&str>) -> Vec<i32> {
    get_columns(get_rows(data))
        .iter()
        .map(most_common_bit)
        .collect()
}

pub fn epsilon(data: Vec<&str>) -> i32 {
    to_int(gamma(data).iter().map(|num| 1 - num).collect())
}

pub fn oxygen(data: Vec<&str>) -> i32 {
    reduce_bins(data, &|char1, char2| char1 == char2)
}

pub fn co2(data: Vec<&str>) -> i32 {
    reduce_bins(data, &|char1, char2| char1 != char2)
}

pub fn to_int(nums: Vec<i32>) -> i32 {
    let bin_src = nums
        .iter()
        .map(|b| b.to_string())
        .collect::<Vec<String>>()
        .join("");

    i32::from_str_radix(&bin_src[..], 2).unwrap()
}
