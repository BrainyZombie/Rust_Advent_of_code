use std::{collections::HashSet, ops::ControlFlow};

pub fn run<T>(input: &str, _: T) -> Result<String, String>
where
    T: Iterator<Item = String>,
{
    let mut priority_sum = 0;
    input.lines().for_each(|line| {
        let mut set = HashSet::new();
        line.char_indices().try_for_each(|(i, c)| {
            if i < line.len() / 2 {
                set.insert(c);
                return ControlFlow::Continue(());
            } else if set.contains(&c) {
                let priority = if c.is_ascii_uppercase() {
                    c as u32 - 'A' as u32 + 27u32
                } else {
                    c as u32 - 'a' as u32 + 1u32
                };
                priority_sum += priority;
                return ControlFlow::Break(());
            }
            ControlFlow::Continue(())
        });
    });

    Ok(priority_sum.to_string())
}
