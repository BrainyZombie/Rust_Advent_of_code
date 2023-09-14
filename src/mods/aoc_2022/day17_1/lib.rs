struct State {
    rock_pos: (usize, usize),
    rock: usize,
    grid: [[bool; 6067]; 8],
    tallest_part: usize,
}
enum Instruction {
    Left,
    Right,
}

impl From<char> for Instruction {
    fn from(c: char) -> Self {
        match c {
            '>' => Instruction::Right,
            '<' => Instruction::Left,
            _ => panic!("Invalid instruction"),
        }
    }
}
pub fn run<T>(input: &str, _: T) -> Result<String, String>
where
    T: Iterator<Item = String>,
{
    let rocks = [
        [
            [true, false, false, false],
            [true, false, false, false],
            [true, false, false, false],
            [true, false, false, false],
        ],
        [
            [false, true, false, false],
            [true, true, true, false],
            [false, true, false, false],
            [false, false, false, false],
        ],
        [
            [true, false, false, false],
            [true, false, false, false],
            [true, true, true, false],
            [false, false, false, false],
        ],
        [
            [true, true, true, true],
            [false, false, false, false],
            [false, false, false, false],
            [false, false, false, false],
        ],
        [
            [true, true, false, false],
            [true, true, false, false],
            [false, false, false, false],
            [false, false, false, false],
        ],
    ];

    let rock_dimensions: [(usize, usize); 5] = [(4, 1), (3, 3), (3, 3), (1, 4), (2, 2)];

    let mut instructions = input.chars().cycle();
    let mut state: State = State {
        rock_pos: (2, 6062),
        rock: 0,
        grid: [[false; 6067]; 8],
        tallest_part: 6066,
    };
    for i in 0..8 {
        state.grid[i][6066] = true;
    }

    for _ in 0..2022 {
        'outer: loop {
            let instruction: Instruction = instructions.next().unwrap().into();

            'test: {
                match instruction {
                    Instruction::Left => {
                        if state.rock_pos.0 != 0 {
                            let new_pos = (state.rock_pos.0 - 1, state.rock_pos.1);
                            for i in 0..rock_dimensions[state.rock].0 {
                                for j in 0..rock_dimensions[state.rock].1 {
                                    if rocks[state.rock][i][j]
                                        && state.grid[new_pos.0 + i][new_pos.1 - j]
                                    {
                                        break 'test;
                                    }
                                }
                            }
                            state.rock_pos.0 -= 1;
                        }
                    }
                    Instruction::Right => {
                        if state.rock_pos.0 + rock_dimensions[state.rock].0 != 7 {
                            let new_pos = (state.rock_pos.0 + 1, state.rock_pos.1);
                            for i in 0..rock_dimensions[state.rock].0 {
                                for j in 0..rock_dimensions[state.rock].1 {
                                    if rocks[state.rock][i][j]
                                        && state.grid[new_pos.0 + i][new_pos.1 - j]
                                    {
                                        break 'test;
                                    }
                                }
                            }
                            state.rock_pos.0 += 1;
                        }
                    }
                }
            }
            //now move down 1
            let new_pos = (state.rock_pos.0, state.rock_pos.1 + 1);

            for i in 0..rock_dimensions[state.rock].0 {
                for j in 0..rock_dimensions[state.rock].1 {
                    if rocks[state.rock][i][j] && state.grid[new_pos.0 + i][new_pos.1 - j] {
                        for i1 in 0..rock_dimensions[state.rock].0 {
                            for j1 in 0..rock_dimensions[state.rock].1 {
                                state.grid[state.rock_pos.0 + i1][state.rock_pos.1 - j1] = rocks
                                    [state.rock][i1][j1]
                                    || state.grid[state.rock_pos.0 + i1][state.rock_pos.1 - j1];
                            }
                        }
                        state.tallest_part = state
                            .tallest_part
                            .min(state.rock_pos.1 - rock_dimensions[state.rock].1 + 1);
                        state.rock = (state.rock + 1) % 5;
                        println!();
                        state.rock_pos = (2, state.tallest_part - 4);
                        break 'outer;
                    }
                }
            }
            state.rock_pos.1 += 1;
        }
    }

    // for j in 6000..state.grid[0].len() {
    //     for i in 0..state.grid.len() {
    //         print!("{}", if state.grid[i][j] { '#' } else { '.' })
    //     }
    //     println!();
    // }
    state.rock_pos.1 += 1;
    Ok((6066 - state.tallest_part).to_string())
}
