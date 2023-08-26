enum Instruction {
    Add {
        number: isize,
        remaining_cycles: usize,
    },
    Noop,
}

fn read_instructions(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| line.split(' ').collect::<Vec<&str>>())
        .map(|tokens| match tokens[0] {
            "noop" => Instruction::Noop,
            "addx" => Instruction::Add {
                number: tokens[1].parse().unwrap(),
                remaining_cycles: 2,
            },
            _ => panic!(),
        })
        .collect()
}

fn calculate_signal_strength_sum(mut instructions: Vec<Instruction>) -> isize {
    let mut x = 1isize;
    let mut signal_strength_sum = 0;
    let mut idx = 0;

    for cycle in 1..=220isize {
        if matches!(cycle, 20 | 60 | 100 | 140 | 180 | 220) {
            signal_strength_sum += x * cycle;
        }
        if let Instruction::Add {
            number,
            remaining_cycles,
        } = instructions[idx]
        {
            if remaining_cycles == 1 {
                idx += 1;
                x += number;
            } else {
                instructions[idx] = Instruction::Add {
                    number,
                    remaining_cycles: remaining_cycles - 1,
                }
            }
        } else {
            idx += 1;
        }
    }

    signal_strength_sum
}

pub fn run<T>(input: &str, _: T) -> Result<String, String>
where
    T: Iterator<Item = String>,
{
    let instructions = read_instructions(input);
    let signal_strength_sum = calculate_signal_strength_sum(instructions);
    Ok(signal_strength_sum.to_string())
}
