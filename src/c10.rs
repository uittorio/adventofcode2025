use std::cmp::{max, min};

use crate::utils;

#[derive(Debug)]
pub struct Data {
    fresh_ingredients: Vec<(usize, usize)>,
}

pub fn run() -> usize {
    let result = utils::challenge_file("09");

    let mut raw = result.split("\n\n");
    let fresh_ingredients = raw
        .next()
        .unwrap_or("")
        .split('\n')
        .map(|x| {
            let start_and_end = x.split('-').collect::<Vec<&str>>();
            return (
                start_and_end[0].parse::<usize>().unwrap(),
                start_and_end[1].parse::<usize>().unwrap(),
            );
        })
        .collect::<Vec<(usize, usize)>>();

    let data = Data { fresh_ingredients };

    return find_total_fresh_ingredients(&data);
}
fn find_total_fresh_ingredients(data: &Data) -> usize {
    let mut analyzed: Vec<(usize, usize)> = vec![];

    for id_range in data.fresh_ingredients.iter() {
        if analyzed.len() == 0 {
            analyzed.push(id_range.clone());
            continue;
        }

        let mut to_analyze = vec![id_range.clone()];
        println!("Range {:?}", id_range);
        for c in analyzed.iter() {
            println!("{:?}", to_analyze);
            to_analyze = outside_ranges2(&to_analyze, c);
        }

        if to_analyze.len() > 0 {
            analyzed.append(&mut to_analyze);
        }
    }

    let sum = analyzed.iter().map(|x| (x.1 - x.0) + 1).sum::<usize>();
    sum
}

fn outside_ranges2(a: &Vec<(usize, usize)>, b: &(usize, usize)) -> Vec<(usize, usize)> {
    let mut result = Vec::new();
    for i in a.iter() {
        result.append(&mut outside_ranges(i, b));
    }

    result
}

fn outside_ranges(a: &(usize, usize), b: &(usize, usize)) -> Vec<(usize, usize)> {
    let mut result = Vec::new();

    if b.0 > a.0 {
        result.push((a.0, min(a.1, b.0 - 1)))
    }

    if b.1 < a.1 && b.0 > a.0 {
        result.push((b.1 + 1, a.1))
    }

    if b.1 < a.1 && b.0 <= a.0 {
        result.push((max(a.0, b.1 + 1), a.1))
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn outside_ranges_test_all_inside_boundary_left() {
        let result = outside_ranges(&(3, 4), &(3, 10));
        assert_eq!(result.len(), 0);
    }

    #[test]
    fn outside_ranges_test_all_inside_boundary_right() {
        let result = outside_ranges(&(4, 10), &(3, 10));
        assert_eq!(result.len(), 0);
    }

    #[test]
    fn outside_ranges_test_all_inside_boundary_left_and_right() {
        let result = outside_ranges(&(3, 10), &(3, 10));
        assert_eq!(result.len(), 0);
    }

    #[test]
    fn outside_ranges_test_full_left() {
        let result = outside_ranges(&(1, 2), &(3, 5));

        println!("{:?}", result);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], (1, 2));
    }

    #[test]
    fn outside_ranges_test_left_boundary_left() {
        let result = outside_ranges(&(1, 3), &(3, 5));

        println!("{:?}", result);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], (1, 2));
    }

    #[test]
    fn outside_ranges_test_left_boundary_right() {
        let result = outside_ranges(&(1, 5), &(3, 5));

        println!("{:?}", result);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], (1, 2));
    }

    #[test]
    fn outside_ranges_test_left_within() {
        let result = outside_ranges(&(1, 4), &(3, 5));

        println!("{:?}", result);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], (1, 2));
    }

    #[test]
    fn outside_ranges_test_full_right() {
        let result = outside_ranges(&(6, 10), &(3, 5));

        println!("{:?}", result);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], (6, 10));
    }

    #[test]
    fn outside_ranges_test_right_boundary_right() {
        let result = outside_ranges(&(5, 10), &(3, 5));

        println!("{:?}", result);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], (6, 10));
    }

    #[test]
    fn outside_ranges_test_right_boundary_left() {
        let result = outside_ranges(&(3, 10), &(3, 5));

        println!("{:?}", result);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], (6, 10));
    }

    #[test]
    fn outside_ranges_test_right_boundary_within() {
        let result = outside_ranges(&(4, 10), &(3, 5));

        println!("{:?}", result);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], (6, 10));
    }

    #[test]
    fn outside_ranges_test_left_and_right() {
        let result = outside_ranges(&(1, 6), &(3, 5));

        println!("{:?}", result);
        assert_eq!(result.len(), 2);
        assert_eq!(result[0], (1, 2));
        assert_eq!(result[1], (6, 6));
    }
}
