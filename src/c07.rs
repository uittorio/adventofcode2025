use crate::utils;

#[derive(Debug)]
pub enum Point {
    Paper,
    Empty,
}
pub fn run() -> u64 {
    let result = utils::challenge_file("07");

    let grid = result
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

    find_accessibile_rolls(grid)
}
fn find_accessibile_rolls(grid: Vec<Vec<Point>>) -> u64 {
    let mut total = 0;
    for (column_i, column) in grid.iter().enumerate() {
        for (row_i, cell) in column.iter().enumerate() {
            match cell {
                Point::Paper => {
                    if check_adjacent(&grid, column_i, row_i) {
                        total += 1;
                    }
                }
                Point::Empty => {}
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
