use std::{collections::vec_deque::VecDeque, ops::ControlFlow};

fn swap_letter(c: u8) -> u8 {
    match c {
        b'S' => b'a',
        b'E' => b'z',
        c => c,
    }
}

fn bfs(lines: &Vec<&[u8]>) -> usize {
    let mut start: Option<(usize, usize)> = None;

    let _ = lines.iter().enumerate().try_for_each(|(i, line)| {
        return line.iter().enumerate().try_for_each(|(j, c)| {
            if *c == b'E' {
                start = Some((i, j));
                return Err(ControlFlow::<(), ()>::Break(()));
            };

            Ok(())
        });
    });

    let start = start.unwrap();

    let mut shortest_paths: Vec<Vec<bool>> = vec![vec![false; lines[0].len()]; lines.len()];
    let mut next_idx = VecDeque::<(usize, usize, usize)>::new();
    next_idx.push_front((start.0, start.1, 0));

    loop {
        let c = next_idx.pop_back().unwrap();
        let c_letter = swap_letter(lines[c.0][c.1]);
        if c_letter == b'a' {
            break c.2;
        }

        if c.1 > 0 {
            let u = (c.0, c.1 - 1);
            if !shortest_paths[u.0][u.1] && c_letter <= swap_letter(lines[u.0][u.1]) + 1 {
                shortest_paths[u.0][u.1] = true;
                next_idx.push_front((u.0, u.1, c.2 + 1));
            }
        }

        if c.1 < lines[0].len() - 1 {
            let d = (c.0, c.1 + 1);
            if !shortest_paths[d.0][d.1] && c_letter <= swap_letter(lines[d.0][d.1]) + 1 {
                shortest_paths[d.0][d.1] = true;
                next_idx.push_front((d.0, d.1, c.2 + 1));
            }
        }

        if c.0 > 0 {
            let l = (c.0 - 1, c.1);
            if !shortest_paths[l.0][l.1] && c_letter <= swap_letter(lines[l.0][l.1]) + 1 {
                shortest_paths[l.0][l.1] = true;
                next_idx.push_front((l.0, l.1, c.2 + 1));
            }
        }

        if c.0 < lines.len() - 1 {
            let r = (c.0 + 1, c.1);
            if !shortest_paths[r.0][r.1] && c_letter <= swap_letter(lines[r.0][r.1]) + 1 {
                shortest_paths[r.0][r.1] = true;
                next_idx.push_front((r.0, r.1, c.2 + 1));
            }
        }
    }
}

pub fn run<T>(input: &str, _: T) -> Result<String, String>
where
    T: Iterator<Item = String>,
{
    let lines: Vec<&[u8]> = input.lines().map(|line: &str| line.as_bytes()).collect();

    let shortest_path = bfs(&lines);
    Ok(shortest_path.to_string())
}
