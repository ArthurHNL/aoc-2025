use std::ops::Range;

fn main() {
    let input = include_str!("./input.txt");
    let id_ranges = parse_input(input);
    let invalid_ids_sum_naive = calc_invalid_id_sum_naive(&id_ranges);
    println!("Sum of invalid IDs (naive): {}", invalid_ids_sum_naive);
}

fn parse_input(input: &str) -> Vec<Range<u64>> {
    input.split(",").map(|str| str.trim()).map(parse_input_range_str).collect()
}

fn parse_input_range_str(range_str: &str) -> Range<u64> {
    // input ranges are inclusive-inclusive, rust Range is inclusive-exclusive, so add 1 to the end
    let mut range_str_parts = range_str.split("-");
    let lower = range_str_parts.next().unwrap().parse::<u64>().unwrap();
    let upper = range_str_parts.next().unwrap().parse::<u64>().unwrap() + 1;
    return lower..upper;
}

fn calc_invalid_id_sum_naive(id_ranges: &Vec<Range<u64>>) -> u128 {
    let mut sum_invalid_ids: u128 = 0;
    for id_range in id_ranges {
        for id in id_range.clone() {
            match is_valid_id_naive(id) {
                true => (),
                false => {
                    //println!("Invalid ID found: {}", id);
                    sum_invalid_ids += id as u128;
                },
            }
        }
    }

    sum_invalid_ids
}

fn is_valid_id_naive(id: u64) -> bool {
    // An id is invalid when its decimal representation is made of the same
    // sequence of digits twice.
    let id_str = id.to_string();

    // If the number of digits is odd, it can't be made of two same sequences.
    match id_str.len() % 2 {
        0 => (),
        _ => return true,
    }

    let split_idx = id_str.len() / 2;

    id_str[0..split_idx] != id_str[split_idx..]
}
