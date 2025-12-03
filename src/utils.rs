use std::fs;

pub fn challenge_file(challenge: &str) -> String {
    let var_name = format!("src/inputs/{}", challenge);
    fs::read_to_string(var_name + ".txt").unwrap()
}
