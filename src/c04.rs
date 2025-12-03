use crate::utils;

pub fn run() -> i64 {
    let result = utils::challenge_file("03");
    let mut invalid_ids: Vec<i64> = vec![];
    for range in result.split(',').filter(|x| !x.is_empty()) {
        let parts: Vec<&str> = range
            .split('-')
            .map(|x| x.trim_ascii())
            .filter(|x| !x.is_empty())
            .collect();
        let first_id = parts[0];
        let second_id = parts[1];
        invalid_ids.append(&mut find_invalid_ids(first_id, second_id));
    }

    invalid_ids.iter().sum()
}
fn find_invalid_ids(first: &str, second: &str) -> Vec<i64> {
    let first_n = first.parse::<i64>().unwrap();
    let second_n = second.parse::<i64>().unwrap();
    let mut invalid_ids: Vec<i64> = vec![];

    for n in first_n..=second_n {
        let n_str = n.to_string();

        let mut current_match = String::new();
        let mut current_index = 0;
        let mut matching = false;

        while n_str.len() != current_index {
            let take = if current_match.len() == 0 {
                1
            } else {
                current_match.len()
            };

            let to_check: String = n_str.chars().skip(current_index).take(take).collect();

            if to_check != current_match {
                matching = false;
                current_match = n_str.chars().take(current_index + 1).collect();
                current_index += 1;
            } else {
                matching = true;
                current_index += current_match.len();
            }
        }

        if matching {
            invalid_ids.push(n);
        }
    }
    invalid_ids
}
