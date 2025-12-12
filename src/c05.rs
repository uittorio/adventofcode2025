use crate::utils;

pub fn run() -> u32 {
    let result = utils::challenge_file("05");
    let mut largest_joltages: Vec<u32> = vec![];
    for bank in result.split('\n').filter(|x| !x.is_empty()) {
        largest_joltages.push(find_largest_joltage(bank));
    }

    largest_joltages.iter().sum()
}
fn find_largest_joltage(bank: &str) -> u32 {
    let mut largest_joltage = 0;
    for (i, n1) in bank.chars().enumerate() {
        for n2 in bank.chars().skip(i + 1) {
            let combination = format!("{}{}", n1, n2);

            let combination_n = combination.parse::<u32>().unwrap();

            if combination_n > largest_joltage {
                largest_joltage = combination_n;
            }
        }
    }

    return largest_joltage;
}
