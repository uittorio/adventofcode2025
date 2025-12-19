use std::collections::HashMap;

use crate::utils;

pub fn run() -> usize {
    let result = utils::challenge_file("13_test");
    let grid = result
        .split("\n")
        .map(|x| x.trim())
        .filter(|x| !x.is_empty())
        .map(|x| x.split("").filter(|x| !x.is_empty()).collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();

    find_splits(grid)
}

fn find_splits(grid: Vec<Vec<&str>>) -> usize {
    let beginning_point = find_beginning(&grid);
    let mut map = HashMap::<(usize, usize), usize>::new();
    return iterate(&grid, beginning_point, &mut map);
}

fn iterate(
    grid: &Vec<Vec<&str>>,
    beginning_point: (usize, usize),
    map: &mut HashMap<(usize, usize), usize>,
) -> usize {
    if beginning_point.0 == grid.len() - 1 {
        return 1;
    }

    let next_point = (beginning_point.0 + 1, beginning_point.1);

    if grid[next_point.0][next_point.1] == "^" {
        let left = (next_point.0, next_point.1 - 1);
        let right = (next_point.0, next_point.1 + 1);

        let left_result = match map.get(&left) {
            Some(v) => *v,
            None => {
                let left_r = iterate(grid, left, map);
                map.insert(left, left_r);
                left_r
            }
        };

        let right_result = match map.get(&right) {
            Some(v) => *v,
            None => {
                let right_r = iterate(grid, right, map);
                map.insert(right, right_r);
                right_r
            }
        };

        return left_result + right_result;
    }

    return iterate(grid, next_point, map);
}

fn find_beginning(grid: &Vec<Vec<&str>>) -> (usize, usize) {
    for (row_i, row) in grid.iter().enumerate() {
        for (column_i, cell) in row.iter().enumerate() {
            if cell == &"S" {
                return (row_i, column_i);
            }
        }
    }

    (0, 0)
}
