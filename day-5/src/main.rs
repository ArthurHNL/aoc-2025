use std::collections::HashSet;

fn main() {
    let input = include_str!("./input.txt");
    let (fresh_ids, requested_ids) = parse_input(input);

    println!("Fresh ids: {:?}", fresh_ids);
    println!("Requested ids: {:?}", requested_ids);

    println!(
        "Count of requested ids that are fresh (naive): {}",
        requested_fresh_ids_count_naive(&fresh_ids, &requested_ids)
    );
}

fn parse_fresh_id_set_line(input_line: &str) -> (usize, usize) {
    input_line
        .split_once('-')
        .map(|(lower, upper)| {
            (
                lower.parse::<usize>().unwrap(),
                upper.parse::<usize>().unwrap(),
            )
        })
        .unwrap()
}

// The input is modeled as a pair of hash sets, where the first set contains the fresh ingredient
// ids and the second set contains the requested ids.
fn parse_input(input: &str) -> (HashSet<(usize, usize)>, HashSet<usize>) {
    let trimmed_input: Vec<&str> = input.lines().map(|l| l.trim()).collect();
    let split_ix = trimmed_input
        .iter()
        .enumerate()
        .find(|(_, line)| line.is_empty())
        .unwrap()
        .0;
    println!("split_ix: {}", split_ix);
    let input_ids_lines = trimmed_input[..split_ix].to_vec();
    println!("split lines: {}", input_ids_lines.len());
    let input_ids = input_ids_lines
        .iter()
        .map(|line| parse_fresh_id_set_line(line))
        .collect::<HashSet<(usize, usize)>>();
    println!("input ids: {}", input_ids.len());

    let input_requests = trimmed_input[split_ix + 1..]
        .iter()
        .map(|line| line.parse::<usize>().unwrap())
        .collect::<HashSet<usize>>();

    (input_ids, input_requests)
}

fn requested_fresh_ids_count_naive(
    fresh_ids: &HashSet<(usize, usize)>,
    requested_ids: &HashSet<usize>,
) -> usize {
    requested_ids
        .iter()
        .filter(|id| {
            fresh_ids
                .iter()
                .any(|(lower, upper)| **id >= *lower && **id <= *upper)
        })
        .count()
}
