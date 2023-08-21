#[derive(PartialEq)]
enum Move {
    Rock,
    Paper,
    Scissor,
}

fn win_score(opponent_move: &Move, our_move: &Move) -> i32 {
    if our_move == opponent_move {
        return 3;
    }
    if (*our_move == Move::Rock && *opponent_move == Move::Scissor)
        || (*our_move == Move::Paper && *opponent_move == Move::Rock)
        || (*our_move == Move::Scissor && *opponent_move == Move::Paper)
    {
        return 6;
    }
    0
}

fn move_score(our_move: &Move) -> i32 {
    match our_move {
        Move::Rock => 1,
        Move::Paper => 2,
        Move::Scissor => 3,
    }
}

pub fn run<T>(input: &str, _: T) -> Result<String, String>
where
    T: Iterator<Item = String>,
{
    let mut total_score = 0;

    input
        .lines()
        .map(|line| {
            let opponent_move = match line.chars().next() {
                Some('A') => Move::Rock,
                Some('B') => Move::Paper,
                Some('C') => Move::Scissor,
                _ => panic!("Invalid Move!"),
            };
            let our_move = match line.chars().nth(2) {
                Some('X') => Move::Rock,
                Some('Y') => Move::Paper,
                Some('Z') => Move::Scissor,
                _ => panic!("Invalid Move!"),
            };
            (opponent_move, our_move)
        })
        .for_each(|(opponent_move, our_move)| {
            total_score += win_score(&opponent_move, &our_move) + move_score(&our_move)
        });
    Ok(total_score.to_string())
}
