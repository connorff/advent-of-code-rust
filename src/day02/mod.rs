use std::{fs, ops::Index};

type MvFn = fn(i32, Vec<i32>) -> Vec<i32>;
pub fn get_moved<'a, T>(size: usize, commands: T, forward: MvFn, up: MvFn, down: MvFn) -> Vec<i32>
where
    T: Iterator<Item = Vec<&'a str>>,
{
    commands.fold(vec![0; size], |res, cmd| {
        let delta = cmd.index(1).parse::<i32>().unwrap();

        let point = match cmd.index(0) {
            &"forward" => forward(delta, res.clone()),
            &"up" => up(delta, res.clone()),
            &"down" => down(delta, res.clone()),
            _ => vec![0; size],
        };

        point.iter().zip(res.iter()).map(|(&b, &v)| b + v).collect()
    })
}

pub fn travel() -> (i32, i32) {
    let data = fs::read_to_string("src/day02/inputs.txt").expect("Unable to read file");
    let split_commands = data
        .split('\n')
        .map(|cmd| cmd.split(' ').collect::<Vec<&str>>());

    let moved = get_moved(
        2,
        split_commands.clone(),
        |d, _| vec![d, 0],
        |d, _| vec![0, -d],
        |d, _| vec![0, d],
    );

    let moved_aim = get_moved(
        3,
        split_commands.clone(),
        |d, res| vec![d, d * res[2], 0],
        |d, _| vec![0, 0, -d],
        |d, _| vec![0, 0, d],
    );

    (moved[0] * moved[1], moved_aim[0] * moved_aim[1])
}
