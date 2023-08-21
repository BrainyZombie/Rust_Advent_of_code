#[derive(PartialEq)]
enum Move {
    Rock,
    Paper,
    Scissor,
}

enum MatchResult {
    Win,
    Draw,
    Lose,
}

fn win_score(match_result: &MatchResult) -> i32 {
    match match_result {
        MatchResult::Win => 6,
        MatchResult::Draw => 3,
        MatchResult::Lose => 0,
    }
}

fn move_score(opponent_move: &Move, match_result: &MatchResult) -> i32 {
    let move_offset = match opponent_move {
        Move::Rock => 0,
        Move::Paper => 1,
        Move::Scissor => 2,
    };
    let result_offset = match match_result {
        MatchResult::Win => 1,
        MatchResult::Draw => 0,
        MatchResult::Lose => 2,
    };
    (move_offset + result_offset) % 3 + 1
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
            let match_result = match line.chars().nth(2) {
                Some('X') => MatchResult::Lose,
                Some('Y') => MatchResult::Draw,
                Some('Z') => MatchResult::Win,
                _ => panic!("Invalid Move!"),
            };
            (opponent_move, match_result)
        })
        .for_each(|(opponent_move, match_result)| {
            total_score += win_score(&match_result) + move_score(&opponent_move, &match_result)
        });
    Ok(total_score.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_score() {
        assert_eq!(
            win_score(&MatchResult::Lose) + move_score(&Move::Rock, &MatchResult::Lose),
            3
        );
        assert_eq!(
            win_score(&MatchResult::Draw) + move_score(&Move::Rock, &MatchResult::Draw),
            4
        );
        assert_eq!(
            win_score(&MatchResult::Win) + move_score(&Move::Rock, &MatchResult::Win),
            8
        );

        assert_eq!(
            win_score(&MatchResult::Lose) + move_score(&Move::Paper, &MatchResult::Lose),
            1
        );
        assert_eq!(
            win_score(&MatchResult::Draw) + move_score(&Move::Paper, &MatchResult::Draw),
            5
        );
        assert_eq!(
            win_score(&MatchResult::Win) + move_score(&Move::Paper, &MatchResult::Win),
            9
        );

        assert_eq!(
            win_score(&MatchResult::Lose) + move_score(&Move::Scissor, &MatchResult::Lose),
            2
        );
        assert_eq!(
            win_score(&MatchResult::Draw) + move_score(&Move::Scissor, &MatchResult::Draw),
            6
        );
        assert_eq!(
            win_score(&MatchResult::Win) + move_score(&Move::Scissor, &MatchResult::Win),
            7
        );
    }
}
