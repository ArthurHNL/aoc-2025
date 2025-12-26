use std::cmp::max;
use transpose::transpose_inplace;

fn main() {
    let input = include_str!("./input.txt");
    let problems = parse_input(input);
    println!("Parsed problems: {:?}", problems);
    println!(
        "Total solution (naive): {}",
        problems.iter().map(calculate_problem).sum::<i64>()
    );
}

#[derive(Debug)]
enum CephalopodMathOperation {
    Add,
    Multiply,
}

#[derive(Debug)]
struct CephalopodMathProblem {
    values: Vec<i64>,
    operation: CephalopodMathOperation,
}

fn parse_input(input: &str) -> Vec<CephalopodMathProblem> {
    let input_non_empty = input
        .lines()
        .filter(|l| !l.is_empty())
        .collect::<Vec<&str>>();
    let input_cells_per_row = input_non_empty
        .iter()
        .map(|l| l.split_whitespace().collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();
    let mut input_cells = input_cells_per_row
        .iter()
        .flatten()
        .cloned()
        .collect::<Vec<&str>>();

    let num_input_rows = input_non_empty.len();
    let num_input_cols = input_cells_per_row[0].len();

    transpose_inplace(
        &mut input_cells,
        &mut vec![""; max(num_input_rows, num_input_cols)],
        num_input_cols,
        num_input_rows,
    );

    input_cells
        .chunks(num_input_rows)
        .map(|column| {
            let input_strs = column[0..column.len() - 1].to_vec();
            let operand_str = column.last().unwrap();

            CephalopodMathProblem {
                values: input_strs
                    .iter()
                    .map(|s| s.parse::<i64>().unwrap())
                    .collect::<Vec<i64>>(),
                operation: match *operand_str {
                    "+" => CephalopodMathOperation::Add,
                    "*" => CephalopodMathOperation::Multiply,
                    _ => panic!("Unknown operand string: {}", operand_str),
                },
            }
        })
        .collect::<Vec<CephalopodMathProblem>>()
}

fn calculate_problem(problem: &CephalopodMathProblem) -> i64 {
    match problem.operation {
        CephalopodMathOperation::Add => problem.values.iter().sum(),
        CephalopodMathOperation::Multiply => problem.values.iter().product(),
    }
}
