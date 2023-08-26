use std::ops::Sub;

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

fn render_screen(mut instructions: Vec<Instruction>) -> String {
    let mut x = 1isize;
    let mut idx = 0;
    let mut frames: String = String::from("");

    for cycle in 0.. {
        if cycle % 240 == 0 {
            frames.push_str("\n");
        }
        if cycle % 40 == 0 {
            frames.push_str("\n");
        }
        if idx == instructions.len() {
            break;
        }

        if (cycle % 40).sub(&x).abs() <= 1 {
            frames.push_str("##");
        } else {
            frames.push_str("  ");
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

    frames
}

pub fn run<T>(input: &str, _: T) -> Result<String, String>
where
    T: Iterator<Item = String>,
{
    let instructions = read_instructions(input);
    let frames = render_screen(instructions);
    Ok(format!("\n{frames}"))
}
