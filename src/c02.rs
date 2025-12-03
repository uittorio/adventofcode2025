use crate::utils;

pub fn run() -> usize {
    let result = utils::challenge_file("01");
    let mut dial = 50;
    let mut output: usize = 0;

    for instruction in result.split("\n").filter(|x| !x.is_empty()) {
        let (direction, movement) = instruction.split_at(1);

        let mut movement_number = movement.parse::<isize>().unwrap();

        if direction == "L" {
            while movement_number != 0 {
                if movement_number > dial {
                    movement_number = movement_number - dial;
                    if movement_number != 0 && dial != 0 {
                        output = output + 1;
                    }
                    dial = 100;
                } else {
                    dial = dial - movement_number;
                    movement_number = 0;
                }
            }
        }

        if direction == "R" {
            while movement_number != 0 {
                if movement_number + dial > 99 {
                    movement_number = movement_number - (100 - dial);

                    if movement_number != 0 {
                        output = output + 1;
                    }

                    dial = 0;
                } else {
                    dial = dial + movement_number;
                    movement_number = 0;
                }
            }
        }

        if dial == 0 {
            output = output + 1;
        }
    }

    output
}
