use crate::utils;

#[derive(Debug)]
pub enum Operation {
    Add,
    Multiply,
}

#[derive(Debug)]
pub struct Data {
    problems: Vec<Vec<usize>>,
    operations: Vec<Vec<Operation>>,
}

pub fn run() -> usize {
    let result = utils::challenge_file("11");

    let problems = result
        .split('\n')
        .take_while(|x| !x.starts_with("*") && !x.starts_with("+"))
        .map(|x| {
            x.split(' ')
                .map(|x| x.trim())
                .filter(|x| !x.is_empty())
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>();

    let operations = result
        .split('\n')
        .skip_while(|x| !x.starts_with("*") && !x.starts_with("+"))
        .map(|row| {
            row.split(' ')
                .map(|x| x.trim())
                .filter(|x| !x.is_empty())
                .map(|x| {
                    if x == "+" {
                        return Operation::Add;
                    }

                    if x == "*" {
                        return Operation::Multiply;
                    }

                    panic!("invalid operation")
                })
                .collect::<Vec<Operation>>()
        })
        .collect::<Vec<Vec<Operation>>>();

    let data = Data {
        problems,
        operations,
    };

    resolve_problems(data)
}

fn resolve_problems(data: Data) -> usize {
    let mut results = vec![];
    // for each row
    //
    for column in 0..data.problems.len() {
        for row in 0..data.problems[column].len() {
            let operation = data.operations.get(0).unwrap().get(row).unwrap();

            match operation {
                Operation::Add => {
                    println!("results add {:?}", results.get(column));

                    if results.get(row).is_none() {
                        results.insert(row, 0);
                    }
                    results[row] += data.problems[column][row]
                }
                Operation::Multiply => {
                    println!("results mutl {:?}", results.get(column));
                    println!("to multiply {:?}", data.problems[column][row]);
                    if results.get(row).is_none() {
                        results.insert(row, 1);
                    }
                    results[row] *= data.problems[column][row]
                }
            }
        }
    }

    results.iter().sum()
}
