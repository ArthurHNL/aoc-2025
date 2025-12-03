fn main() {
    let input = include_str!("./input.txt");
    let battery_banks = parse_input(input);
    println!(
        "Total optimal output joltage (naive): {}",
        total_optimal_output_joltage_naive(&battery_banks)
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
