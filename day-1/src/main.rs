fn main() {
    let input = include_str!("./input.txt");
    let instructions = parse_input(input);
    let passcode = process_instructions(&instructions);
    println!("The code is {}", passcode)
}

enum DialTurnDirection {
    Left,
    Right,
}

struct DialTurnInstruction {
    clicks: u16,
    direction: DialTurnDirection
}

fn parse_dial_turn_instruction(instruction: &str) -> DialTurnInstruction {
    let mut chars_iter = instruction.chars();
    let direction_char = chars_iter.next().expect("Instruction string is empty");
    let direction = match direction_char {
        'L' => DialTurnDirection::Left,
        'R' => DialTurnDirection::Right,
        _ => panic!("Unknown direction character: {}", direction_char)
    };
    let clicks_char = chars_iter.collect::<String>();
    let clicks = clicks_char.parse::<u16>().expect("Failed to parse clicks characters");

    return DialTurnInstruction {
        clicks,
        direction
    };
}

fn parse_input(input: &str) -> Vec<DialTurnInstruction> {
    return input.lines()
        .map(|line| line.trim())
        .map(parse_dial_turn_instruction)
        .collect()
}

fn process_instructions(instructions: &Vec<DialTurnInstruction>) -> u16 {
    let mut dial_position: i32 = 50;
    let mut count_dial_position_0: u16 = 0;

    for instruction in instructions {
        dial_position = get_next_dial_position(&dial_position, instruction);
        if dial_position == 0 {
            count_dial_position_0 += 1;
        }
    }

    return count_dial_position_0;
}

fn get_next_dial_position(current_position: &i32, current_instruction: &DialTurnInstruction) -> i32 {
    let multiplier: i32= match current_instruction.direction {
        DialTurnDirection::Left => -1,
        DialTurnDirection::Right => 1,
    };

    let applied_clicks = multiplier * (current_instruction.clicks as i32);
    let new_position = current_position + applied_clicks;
    return new_position % 100;
}
