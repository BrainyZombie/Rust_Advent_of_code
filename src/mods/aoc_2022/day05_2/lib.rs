struct Move {
    from: usize,
    to: usize,
    count: usize,
}

impl Move {
    fn from<'a, T: Iterator<Item = &'a str>>(lines: &mut T) -> Vec<Move> {
        lines
            .map(|line| {
                let words: Vec<&str> = line.split(' ').collect();
                Move {
                    count: words[1].parse().unwrap(),
                    from: words[3].parse::<usize>().unwrap() - 1,
                    to: words[5].parse::<usize>().unwrap() - 1,
                }
            })
            .collect()
    }
    fn do_move(next_move: &Move, stacks: &mut [Vec<char>]) {
        let stack_from = &mut stacks[next_move.from];
        let move_elems: Vec<char> = stack_from
            .drain(stack_from.len() - next_move.count..)
            .collect();

        let stack_to = &mut stacks[next_move.to];
        stack_to.extend(move_elems);
    }
}

fn generate_stacks<'a, T: Iterator<Item = &'a str>>(
    lines: &mut std::iter::Peekable<T>,
) -> Vec<Vec<char>> {
    assert!((lines.peek().unwrap().len() + 1) % 4 == 0);

    let stack_count = (lines.peek().unwrap().len() + 1) / 4;
    let mut stacks = vec![vec![]; stack_count];

    lines.try_for_each(|line| {
        if (line.len() + 1) % 4 != 0 {
            return None;
        };
        line.as_bytes()
            .chunks(4)
            .enumerate()
            .for_each(|(i, elems)| {
                if (elems[1] as char).is_alphabetic() {
                    stacks[i].push(elems[1] as char);
                }
            });
        Some(())
    });

    stacks.iter_mut().for_each(|stack| stack.reverse());
    stacks
}

pub fn run<T>(input: &str, _: T) -> Result<String, String>
where
    T: Iterator<Item = String>,
{
    let mut lines = input.lines().peekable();
    let mut stacks = generate_stacks(&mut lines);

    let moves = Move::from(&mut lines);
    moves.iter().for_each(|next_move| {
        Move::do_move(next_move, &mut stacks);
    });
    let result = stacks
        .iter_mut()
        .map(|stack: &mut Vec<char>| stack.pop().unwrap())
        .fold(String::from(""), |mut acc: String, next| -> String {
            acc.push(next);
            acc
        });
    Ok(result)
}
