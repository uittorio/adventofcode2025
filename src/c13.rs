use std::collections::HashSet;

use crate::utils;

pub fn run() -> usize {
    let result = utils::challenge_file("13");
    let grid = result
        .split("\n")
        .map(|x| x.trim())
        .filter(|x| !x.is_empty())
        .map(|x| x.split("").filter(|x| !x.is_empty()).collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();

    find_splits(grid)
}

fn find_splits(grid: Vec<Vec<&str>>) -> usize {
    let mut total_splits: HashSet<(usize, usize)> = HashSet::new();
    let beginning_point = find_beginning(&grid);

    iterate(&mut total_splits, &grid, beginning_point);

    total_splits.len()
}

fn find_beginning(grid: &Vec<Vec<&str>>) -> (usize, usize) {
    for (row_i, row) in grid.iter().enumerate() {
        for (cell_i, cell) in row.iter().enumerate() {
            if cell == &"S" {
                return (row_i, cell_i);
            }
        }
    }

    (0, 0)
}

fn iterate(
    total: &mut HashSet<(usize, usize)>,
    grid: &Vec<Vec<&str>>,
    beginning_point: (usize, usize),
) -> () {
    if beginning_point.0 == grid.len() - 1 {
        return;
    }

    let next_point = (beginning_point.0 + 1, beginning_point.1);

    if grid[next_point.0][next_point.1] == "^" {
        let left = (next_point.0, next_point.1 - 1);
        let right = (next_point.0, next_point.1 + 1);

        if total.get(&next_point).is_none() {
            total.insert(next_point);

            iterate(total, grid, left);
            iterate(total, grid, right);
        }
    } else {
        iterate(total, grid, next_point);
    }

    return;
}
