enum FillResult {
    Filled(usize),
    Flows(usize),
}

fn mark_path(grid: &mut [Vec<bool>], from: (usize, usize), to: (usize, usize)) {
    if from.0 != to.0 {
        let from_idx = from.0.min(to.0);
        let to_idx = from.0.max(to.0);
        for idx in from_idx..=to_idx {
            grid[idx][from.1] = true;
        }
    } else {
        let from_idx = from.1.min(to.1);
        let to_idx = from.1.max(to.1);
        for idx in from_idx..=to_idx {
            grid[from.0][idx] = true;
        }
    }
}

fn create_grid(
    paths: &[Vec<(usize, usize)>],
    (x_min, x_max, _, y_max): (usize, usize, usize, usize),
) -> Vec<Vec<bool>> {
    let mut grid = vec![vec![false; x_max - x_min + 1]; y_max + 1];
    let mut prev_coord: Option<(usize, usize)> = None;
    paths.iter().for_each(|path| {
        prev_coord = None;
        path.iter().for_each(|(x2, y2)| {
            if let Some((x1, y1)) = prev_coord {
                mark_path(&mut grid, (y1, x1 - x_min), (*y2, x2 - x_min));
            }
            prev_coord = Some((*x2, *y2));
        })
    });
    grid
}

fn dfs(grid: &mut [Vec<bool>], start: (isize, isize)) -> FillResult {
    if start.1 < 0 || start.0 < 0 {
        return FillResult::Flows(0);
    }
    let start = (start.0 as usize, start.1 as usize);
    if start.1 >= grid.len() || start.0 >= grid[0].len() {
        return FillResult::Flows(0);
    }

    if grid[start.1][start.0] {
        return FillResult::Filled(0);
    }

    let mut result = 0;
    let down_result = dfs(grid, (start.0 as isize, start.1 as isize + 1));
    if let FillResult::Flows(count) = down_result {
        return FillResult::Flows(count + result);
    } else {
        let FillResult::Filled(count) = down_result else {panic!()};
        result += count;
    }

    let down_left_result = dfs(grid, (start.0 as isize - 1, start.1 as isize + 1));
    if let FillResult::Flows(count) = down_left_result {
        return FillResult::Flows(count + result);
    } else {
        let FillResult::Filled(count) = down_left_result else {panic!()};
        result += count;
    }

    let down_right_result = dfs(grid, (start.0 as isize + 1, start.1 as isize + 1));
    if let FillResult::Flows(count) = down_right_result {
        return FillResult::Flows(count + result);
    } else {
        let FillResult::Filled(count) = down_right_result else {panic!()};
        result += count + 1;
    }
    grid[start.1][start.0] = true;
    FillResult::Filled(result)
}

pub fn run<T>(input: &str, _: T) -> Result<String, String>
where
    T: Iterator<Item = String>,
{
    let mut x_bounds = (0, usize::MAX);
    let mut y_bounds = (0, usize::MAX);
    let paths: Vec<Vec<(usize, usize)>> = input
        .lines()
        .map(|path_string| {
            path_string
                .split(" -> ")
                .map(|path_coord_str| {
                    let mut coords = path_coord_str.split(',');
                    let x = coords.next().unwrap().parse().unwrap();
                    let y = coords.next().unwrap().parse().unwrap();

                    x_bounds = (x_bounds.0.max(x), x_bounds.1.min(x));
                    y_bounds = (y_bounds.0.max(y), y_bounds.1.min(y));
                    (x, y)
                })
                .collect()
        })
        .collect();

    let mut grid = create_grid(&paths, (x_bounds.1, x_bounds.0, y_bounds.1, y_bounds.0));

    let FillResult::Flows(result) = dfs(&mut grid, (500-x_bounds.1 as isize, 0)) else {panic!()};
    Ok(result.to_string())
}
