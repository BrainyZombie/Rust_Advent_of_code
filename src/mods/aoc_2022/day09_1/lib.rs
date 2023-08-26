use std::collections::HashSet;

enum Move {
    Right(isize),
    Left(isize),
    Up(isize),
    Down(isize),
}

fn read_moves(input: &str) -> Vec<Move> {
    input
        .lines()
        .map(|line| {
            let tokens: Vec<&str> = line.split(' ').collect();
            match tokens[0] {
                "L" => Move::Left(tokens[1].parse().unwrap()),
                "R" => Move::Right(tokens[1].parse().unwrap()),
                "U" => Move::Up(tokens[1].parse().unwrap()),
                "D" => Move::Down(tokens[1].parse().unwrap()),
                _ => panic!(),
            }
        })
        .collect()
}

fn next_tail_position(
    head_position: (isize, isize),
    tail_position: (isize, isize),
) -> (isize, isize) {
    let diff = (
        (head_position.0 - tail_position.0),
        (head_position.1 - tail_position.1),
    );
    if diff.0.abs() > 1 || diff.1.abs() > 1 {
        let diff = (diff.0.clamp(-1, 1), diff.1.clamp(-1, 1));
        return (tail_position.0 + diff.0, tail_position.1 + diff.1);
    }
    tail_position
}

fn count_positions(moves: &[Move]) -> usize {
    let mut seen_positions = HashSet::<(isize, isize)>::new();
    let mut tail_position = (0, 0);
    let mut head_position = (0, 0);

    seen_positions.insert(tail_position);

    moves.iter().for_each(|next_move| {
        let deltas = match next_move {
            Move::Left(distance) => (-1, 0, distance),
            Move::Right(distance) => (1, 0, distance),
            Move::Up(distance) => (0, 1, distance),
            Move::Down(distance) => (0, -1, distance),
        };

        for _ in 0..*deltas.2 {
            head_position = (head_position.0 + deltas.0, head_position.1 + deltas.1);
            tail_position = next_tail_position(head_position, tail_position);
            seen_positions.insert(tail_position);
        }
    });
    seen_positions.len()
}

pub fn run<T>(input: &str, _: T) -> Result<String, String>
where
    T: Iterator<Item = String>,
{
    let moves = read_moves(input);
    let positions = count_positions(&moves);
    Ok(positions.to_string())
}
