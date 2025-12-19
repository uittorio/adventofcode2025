use crate::utils;

#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy)]
struct Point {
    x: isize,
    y: isize,
}
pub fn run() -> isize {
    let result = utils::challenge_file("17");
    let data = result
        .split("\n")
        .map(|x| x.trim())
        .filter(|x| !x.is_empty())
        .map(|x| {
            x.split(",")
                .filter(|x| !x.is_empty())
                .map(|x| x.parse::<isize>().unwrap())
                .collect::<Vec<isize>>()
        })
        .map(|v| Point { x: v[0], y: v[1] })
        .collect::<Vec<Point>>();

    find(data)
}

fn find(data: Vec<Point>) -> isize {
    let mut biggest_square = 0;
    for i1 in 0..data.len() {
        for i2 in i1..data.len() {
            let x = ((data[i1].x - data[i2].x) + 1).abs();
            let y = ((data[i1].y - data[i2].y) + 1).abs();
            let square = x * y;
            if square > biggest_square {
                biggest_square = square;
            }
        }
    }
    biggest_square
}
