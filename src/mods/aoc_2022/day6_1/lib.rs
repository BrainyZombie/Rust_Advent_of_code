use std::collections::VecDeque;

pub fn run<T>(input: &str, _: T) -> Result<String, String>
where
    T: Iterator<Item = String>,
{
    const UNIQUE_CHARAS: usize = 4;
    let stream = input.bytes();
    let mut char_counts: Vec<u32> = vec![0; 256];
    let mut total_unique = 0;
    let mut prev_chars = VecDeque::<u8>::new();

    let start_idx: usize = stream
        .enumerate()
        .find_map(|(i, b)| {
            char_counts[b as usize] += 1;
            if char_counts[b as usize] == 1 {
                total_unique += 1;
            }
            prev_chars.push_back(b);
            if prev_chars.len() == UNIQUE_CHARAS + 1 {
                let b2 = prev_chars.pop_front().unwrap();
                char_counts[b2 as usize] -= 1;
                if char_counts[b2 as usize] == 0 {
                    total_unique -= 1;
                }
            }
            if total_unique == UNIQUE_CHARAS {
                return Some(i + 1);
            }
            None
        })
        .unwrap();

    Ok(start_idx.to_string())
}
