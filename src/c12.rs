use crate::utils;

#[derive(Debug, Clone)]
pub enum Operation {
    Add,
    Multiply,
}

#[derive(Debug)]
pub struct Problem {
    operation: Operation,
    numbers: Vec<String>,
}

pub fn run() -> usize {
    let result = utils::challenge_file("11");

    let operations_cells = result
        .split('\n')
        .filter(|x| !x.is_empty())
        .last()
        .unwrap()
        .split("")
        .filter(|x| !x.is_empty())
        .collect::<Vec<&str>>();

    let problems_row = result
        .split('\n')
        .filter(|x| !x.is_empty())
        .take_while(|x| !x.starts_with("*") && !x.starts_with("+"))
        .map(|x| x.split("").collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();

    let mut problems: Vec<Problem> = vec![];
    let mut current_problem = Problem {
        numbers: vec![],
        operation: Operation::Add,
    };
    let max_cells = problems_row.iter().map(|x| x.len()).max().unwrap();
    for operation_index in 0..max_cells - 1 {
        let operation_cell = operations_cells.get(operation_index).unwrap_or(&"");

        if operation_cell == &"+" {
            problems.push(current_problem);
            current_problem = Problem {
                numbers: vec![],
                operation: Operation::Add,
            };
        }

        if operation_cell == &"*" {
            problems.push(current_problem);
            current_problem = Problem {
                numbers: vec![],
                operation: Operation::Multiply,
            };
        }

        let mut number_column = String::new();

        for p_row in 0..problems_row.len() {
            if problems_row[p_row].get(operation_index).is_none() {
                continue;
            }
            let number_string = problems_row[p_row][operation_index];

            number_column.push_str(number_string);
        }

        current_problem.numbers.push(number_column);
    }

    problems.push(current_problem);
    problems.reverse();

    problems.iter_mut().for_each(|x| x.numbers.reverse());

    let mut total = 0;

    for problem in problems {
        let mut problem_total = match problem.operation {
            Operation::Add => 0,
            Operation::Multiply => 1,
        };
        for number in problem
            .numbers
            .iter()
            .map(|x| x.trim())
            .filter(|x| !x.is_empty())
        {
            let n = number.parse::<usize>().unwrap();
            match problem.operation {
                Operation::Add => problem_total += n,
                Operation::Multiply => problem_total *= n,
            }
        }

        total += problem_total;
    }

    total
}
