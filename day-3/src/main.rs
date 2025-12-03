fn main() {
    let input = include_str!("./input.txt");
    let battery_banks = parse_input(input);
    println!(
        "Total optimal output joltage (naive): {}",
        total_optimal_output_joltage_naive(&battery_banks)
    );
    println!(
        "Total optimal output joltage (advanced): {}",
        total_optimal_output_joltage_advanced(&battery_banks)
    );
}

// Because we have to 're-arrange' the batteries by decimal value before interpret them as a
// number, we model a battery bank as a single vector of characters.
fn parse_input(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|line| line.trim().chars().collect())
        .collect()
}

fn total_optimal_output_joltage_naive(battery_banks: &Vec<Vec<char>>) -> u64 {
    battery_banks
        .iter()
        .map(optimal_output_joltage_naive_for_bank)
        .map(|joltage| joltage as u64)
        .sum()
}

fn optimal_output_joltage_naive_for_bank(battery_bank: &Vec<char>) -> u32 {
    let mut current_optimum: u32 = 0;

    'outer: for idx_left in 0..battery_bank.len() {
        if idx_left == battery_bank.len() - 1 {
            continue;
        }

        let left = battery_bank[idx_left];

        for idx_right in idx_left + 1..battery_bank.len() {
            let right = battery_bank[idx_right];
            let value = format!("{}{}", left, right).parse::<u32>().unwrap();

            if current_optimum < value {
                current_optimum = value;
            }

            if current_optimum == 99 {
                break 'outer;
            }
        }
    }

    //println!("{}", current_optimum);
    current_optimum
}

fn total_optimal_output_joltage_advanced(battery_banks: &Vec<Vec<char>>) -> u128 {
    battery_banks
        .iter()
        .map(optimal_output_joltage_advanced_for_bank)
        .map(|joltage| joltage as u128)
        .sum()
}

fn optimal_output_joltage_advanced_for_bank(battery_bank: &Vec<char>) -> u64 {
    let mut left_lower_idx = 0;
    let mut target_cells: Vec<char> = Vec::new();
    for target_idx in 0..12 {
        let mut current_max_char: char = '\0';
        let remaining_chars = 12 - target_idx - 1;
        let upper_bound = battery_bank.len() - remaining_chars;

        for current_idx in left_lower_idx..upper_bound {
            if current_max_char < battery_bank[current_idx] {
                current_max_char = battery_bank[current_idx];
                left_lower_idx = current_idx + 1;
            }
        }
        target_cells.push(current_max_char)
    }

    let optimum: u64 = target_cells.iter().collect::<String>().parse().unwrap();
    //println!("{}", optimum);
    optimum
}
