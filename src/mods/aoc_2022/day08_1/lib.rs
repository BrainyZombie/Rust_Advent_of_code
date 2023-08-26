use std::collections::HashSet;

fn create_grid(input: &str) -> Vec<Vec<isize>> {
    Vec::from_iter(input.lines().map(|line| {
        Vec::from_iter(
            line.bytes()
                .map(|num| (num as char).to_digit(10).unwrap() as isize),
        )
    }))
}

fn count(grid: &Vec<Vec<isize>>) -> isize {
    let mut seen = HashSet::new();
    let mut curr_max = 0;
    (0..grid.len()).for_each(|i| {
        curr_max = -1;
        (0..grid[i].len()).for_each(|j| {
            if grid[i][j] > curr_max {
                seen.insert((i, j));
                curr_max = grid[i][j];
            }
        });
        curr_max = -1;
        (0..grid[i].len()).rev().for_each(|j| {
            if grid[i][j] > curr_max {
                seen.insert((i, j));
                curr_max = grid[i][j];
            }
        });
    });
    (0..grid[0].len()).for_each(|j| {
        curr_max = -1;
        (0..grid.len()).for_each(|i| {
            if grid[i][j] > curr_max {
                seen.insert((i, j));
                curr_max = grid[i][j];
            }
        });
        curr_max = -1;
        (0..grid.len()).rev().for_each(|i| {
            if grid[i][j] > curr_max {
                seen.insert((i, j));
                curr_max = grid[i][j];
            }
        });
    });

    seen.len() as isize
}

pub fn run<T>(input: &str, _: T) -> Result<String, String>
where
    T: Iterator<Item = String>,
{
    let grid = create_grid(input);
    let total_count = count(&grid);

    Ok(total_count.to_string())
}
