fn create_grid(input: &str) -> Vec<Vec<usize>> {
    Vec::from_iter(input.lines().map(|line| {
        Vec::from_iter(
            line.bytes()
                .map(|num| (num as char).to_digit(10).unwrap() as usize),
        )
    }))
}

fn cmp_last_seen(
    last_seen_height: &[Option<usize>; 10],
    current_height: usize,
    idx: usize,
) -> usize {
    let mut answer = 0;
    last_seen_height
        .iter()
        .skip(current_height)
        .for_each(|last_seen| {
            if let Some(last_seen) = last_seen {
                answer = answer.max(*last_seen);
            }
        });
    idx - answer
}

fn result(grid: &Vec<Vec<usize>>) -> usize {
    let mut scores: Vec<Vec<usize>> = vec![vec![0; grid[0].len()]; grid.len()];
    let mut last_seen: [Option<usize>; 10] = [None; 10];

    (0..grid.len()).for_each(|i| {
        last_seen = [None; 10];
        (0..grid[i].len()).for_each(|j| {
            scores[i][j] = cmp_last_seen(&last_seen, grid[i][j], j);
            last_seen[grid[i][j]] = Some(j);
        });
        last_seen = [None; 10];
        (0..grid[i].len()).rev().for_each(|j| {
            scores[i][j] *= cmp_last_seen(&last_seen, grid[i][j], grid[i].len() - j - 1);
            last_seen[grid[i][j]] = Some(grid[i].len() - j - 1);
        });
    });
    (0..grid[0].len()).for_each(|j| {
        last_seen = [None; 10];
        (0..grid.len()).for_each(|i| {
            scores[i][j] *= cmp_last_seen(&last_seen, grid[i][j], i);
            last_seen[grid[i][j]] = Some(i);
        });
        last_seen = [None; 10];
        (0..grid.len()).rev().for_each(|i| {
            scores[i][j] *= cmp_last_seen(&last_seen, grid[i][j], grid.len() - i - 1);
            last_seen[grid[i][j]] = Some(grid.len() - i - 1);
        });
    });

    *scores.iter().flatten().max().unwrap_or(&0)
}

pub fn run<T>(input: &str, _: T) -> Result<String, String>
where
    T: Iterator<Item = String>,
{
    let grid = create_grid(input);
    let total_count = result(&grid);

    Ok(total_count.to_string())
}
