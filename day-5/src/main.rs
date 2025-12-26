use std::cmp::max;
use std::collections::HashSet;

fn main() {
    let input = include_str!("./input.txt");
    let (fresh_id_ranges, requested_ids) = parse_input(input);

    println!("Fresh ids: {:?}", fresh_id_ranges);
    println!("Requested ids: {:?}", requested_ids);

    println!(
        "Count of requested ids that are fresh (naive): {}",
        requested_fresh_ids_count_naive(&fresh_id_ranges, &requested_ids)
    );

    println!("Count of fresh ids: {}", fresh_id_count(&fresh_id_ranges));
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

// The input is modeled as a pair with the first being a vector of input id ranges
// and the second a hash set of requested ids.
fn parse_input(input: &str) -> (Vec<(usize, usize)>, HashSet<usize>) {
    let trimmed_input: Vec<&str> = input.lines().map(|l| l.trim()).collect();
    let split_ix = trimmed_input
        .iter()
        .enumerate()
        .find(|(_, line)| line.is_empty())
        .unwrap()
        .0;
    let input_ids_lines = trimmed_input[..split_ix].to_vec();
    let mut input_ids = input_ids_lines
        .iter()
        .map(|line| parse_fresh_id_set_line(line))
        .collect::<Vec<(usize, usize)>>();
    input_ids.sort();

    let input_requests = trimmed_input[split_ix + 1..]
        .iter()
        .map(|line| line.parse::<usize>().unwrap())
        .collect::<HashSet<usize>>();

    (input_ids, input_requests)
}

fn requested_fresh_ids_count_naive(
    fresh_ids: &[(usize, usize)],
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

fn fresh_id_count(sorted_fresh_id_ranges: &[(usize, usize)]) -> u128 {
    let mut id_count: u128 = 0;
    let mut previous_range: Option<(usize, usize)> = None;

    for (lower, upper) in sorted_fresh_id_ranges {
        let current_lower = match previous_range {
            Some((_, prev_upper)) if *lower <= prev_upper => prev_upper + 1,
            _ => *lower,
        };

        if current_lower > *upper {
            println!(
                "Skipping range: {}-{}, current_lower={}, state={}",
                lower, upper, current_lower, id_count
            );
            continue;
        }

        let increment = max((*upper - current_lower) + 1, 0);

        println!(
            "Processing range: {}-{}, current_lower={}, increment={}, state={}",
            lower, upper, current_lower, increment, id_count
        );

        id_count += increment as u128;
        previous_range = Some((*lower, *upper));
    }

    id_count
}
