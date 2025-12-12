use crate::utils;

pub fn run() -> u64 {
    let result = utils::challenge_file("05");
    let mut largest_joltages: Vec<u64> = vec![];
    for bank in result.split('\n').filter(|x| !x.is_empty()) {
        largest_joltages.push(find_largest_joltage(bank));
    }

    largest_joltages.iter().sum()
}
fn find_largest_joltage(bank: &str) -> u64 {
    let mut bank_copy = bank;
    let mut largest_to_check = 9;
    let mut remaining = 12;
    let mut new_list = String::new();

    while remaining != 0 {
        let mut found_index: Option<usize> = None;
        for (i, v) in bank_copy.chars().enumerate() {
            let digit = v.to_digit(10).unwrap();
            if digit == largest_to_check && bank_copy.len() - i > remaining - 1 {
                new_list.push(v);
                remaining -= 1;
                found_index = Some(i);
                break;
            }
        }

        if found_index.is_some() {
            bank_copy = bank_copy.split_at(found_index.unwrap() + 1).1;
            largest_to_check = 9;
        } else {
            largest_to_check -= 1;
        }
    }

    return new_list.parse::<u64>().unwrap();
}
