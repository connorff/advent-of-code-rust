use std::{cell::Cell, fs, ops::Index};

type BoardItem<'a> = (&'a str, Cell<bool>);
type Board<'a> = Vec<Vec<BoardItem<'a>>>;

fn move_wins(board: Board, row: usize, item: usize) -> bool {
    let row_won = board.index(row).iter().all(|item| item.1.get());
    let col_won = board
        .iter()
        .map(|row| row.index(item))
        .all(|item| item.1.get());

    row_won || col_won
}

pub fn bingo() -> i32 {
    let raw_boards = fs::read_to_string("src/day04/boards.txt").expect("Unable to read file");
    let raw_nums = fs::read_to_string("src/day04/nums.txt").expect("Unable to read file");

    let nums: Vec<&str> = raw_nums.split(",").collect();
    let boards: Vec<Board> = raw_boards
        .split("\n\n")
        .map(|board| {
            board
                .split("\n")
                .collect::<Vec<&str>>()
                .iter()
                .map(|row| {
                    row.split(" ")
                        .filter(|item| !item.is_empty())
                        .map(|item| (item, Cell::new(false)))
                        .collect::<Vec<(&str, Cell<bool>)>>()
                })
                .collect()
        })
        .collect();

    let mut wins = vec!();

    for number in nums {
        for (board_index, board) in boards.iter().enumerate() {
            let board_won = wins.iter().any(|(bn, _)| *bn == board_index);

            for (row_index, row) in board.iter().enumerate() {
                for (item_index, item) in row.iter().enumerate() {
                    if number == item.0 {
                        item.1.set(true);

                        if !board_won && move_wins(board.clone(), row_index, item_index) {
                            let unfilled_sum: i32 = board
                                .iter()
                                .flatten()
                                .filter(|(_, b)| !b.get())
                                .map(|(s, _)| s.parse::<i32>().unwrap())
                                .sum();

                            wins.push((board_index, unfilled_sum * item.0.parse::<i32>().unwrap()));
                        }
                    }
                }
            }
        }
    }

    wins.last().unwrap().1
}
