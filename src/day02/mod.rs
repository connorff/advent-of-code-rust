use std::{fs, ops::Index};

pub fn travel() -> (i32, i32) {
    let data = fs::read_to_string("src/day02/inputs.txt").expect("Unable to read file");
    let split_commands = data
        .split('\n')
        .map(|cmd| cmd.split(' ').collect::<Vec<&str>>());

    let total_moved = split_commands.clone().fold((0, 0), |res, cmd| {
        let delta = cmd.index(1).parse::<i32>().unwrap();

        let point = match cmd.index(0) {
            &"forward" => (delta, 0),
            &"up" => (0, -delta),
            &"down" => (0, delta),
            _ => (0, 0),
        };

        (res.0 + point.0, res.1 + point.1)
    });

    let total_moved_with_aim = split_commands.fold((0, 0, 0), |res, cmd| {
        let delta = cmd.index(1).parse::<i32>().unwrap();

        let point = match cmd.index(0) {
            &"forward" => (delta, delta * res.2, 0),
            &"up" => (0, 0, -delta),
            &"down" => (0, 0, delta),
            _ => (0, 0, 0),
        };

        (res.0 + point.0, res.1 + point.1, res.2 + point.2)
    });

    (
        total_moved.0 * total_moved.1,
        total_moved_with_aim.0 * total_moved_with_aim.1,
    )
}
