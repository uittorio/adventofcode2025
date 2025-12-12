use std::vec;

use crate::utils;

#[derive(Debug)]
pub enum Point {
    Paper,
    Empty,
}
pub fn run() -> u64 {
    let result = utils::challenge_file("07");

    let mut grid = result
        .split('\n')
        .filter(|x| !x.trim().is_empty())
        .map(|x| {
            x.chars()
                .map(|x| {
                    if x == '.' {
                        return Point::Empty;
                    }

                    if x == '@' {
                        return Point::Paper;
                    }

                    panic!("invalid symbol");
                })
                .collect()
        })
        .collect::<Vec<Vec<Point>>>();

    let mut previously_removed = 0;
    let mut total_to_remove = 0;

    loop {
        let total = find_accessibile_rolls(&mut grid);
        total_to_remove += total;

        if total == previously_removed {
            break;
        }

        previously_removed = total;
    }

    return total_to_remove;
}
fn find_accessibile_rolls(grid: &mut Vec<Vec<Point>>) -> u64 {
    let mut total = 0;
    let columns_length = grid.len();

    for c in 0..columns_length {
        let rows_length = grid[c].len();

        for r in 0..rows_length {
            match grid[c][r] {
                Point::Paper => {
                    if check_adjacent(&grid, c, r) {
                        grid[c][r] = Point::Empty;
                        total += 1;
                    }
                }
                Point::Empty => {
                    grid[c][r] = Point::Empty;
                }
            }
        }
    }

    total
}

fn check_adjacent(grid: &Vec<Vec<Point>>, column_i: usize, row_i: usize) -> bool {
    let mut number_of_rolls = 0;

    if column_i != 0 && row_i != 0 {
        match grid[column_i - 1][row_i - 1] {
            Point::Paper => number_of_rolls += 1,
            Point::Empty => {}
        }
    }

    if column_i != 0 && row_i != (grid.iter().len() - 1) {
        match grid[column_i - 1][row_i + 1] {
            Point::Paper => number_of_rolls += 1,
            Point::Empty => {}
        }
    }

    if column_i != grid.len() - 1 {
        match grid[column_i + 1][row_i] {
            Point::Paper => number_of_rolls += 1,
            Point::Empty => {}
        }
    }

    if column_i != grid.len() - 1 && row_i != grid.iter().len() - 1 {
        match grid[column_i + 1][row_i + 1] {
            Point::Paper => number_of_rolls += 1,
            Point::Empty => {}
        }
    }

    if row_i != 0 {
        match grid[column_i][row_i - 1] {
            Point::Paper => number_of_rolls += 1,
            Point::Empty => {}
        }
    }

    if column_i != grid.len() - 1 && row_i != 0 {
        match grid[column_i + 1][row_i - 1] {
            Point::Paper => number_of_rolls += 1,
            Point::Empty => {}
        }
    }

    if column_i != 0 {
        match grid[column_i - 1][row_i] {
            Point::Paper => number_of_rolls += 1,
            Point::Empty => {}
        }
    }

    if row_i != grid.iter().len() - 1 {
        match grid[column_i][row_i + 1] {
            Point::Paper => number_of_rolls += 1,
            Point::Empty => {}
        }
    }

    number_of_rolls < 4
}
