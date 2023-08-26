use std::collections::{HashMap, HashSet};

fn char_to_priority(c: char) -> u32 {
    if c.is_ascii_uppercase() {
        c as u32 - 'A' as u32 + 27u32
    } else {
        c as u32 - 'a' as u32 + 1u32
    }
}

pub fn run<T>(input: &str, _: T) -> Result<String, String>
where
    T: Iterator<Item = String>,
{
    let mut priority_sum = 0;
    let lines: Vec<&str> = input.lines().collect();

    let chunked_lines = lines.chunks(3);

    chunked_lines.for_each(|elf_group| {
        let mut map = HashMap::new();
        elf_group.iter().for_each(|line| {
            let mut set = HashSet::new();
            line.chars().for_each(|c| {
                if !set.contains(&c) {
                    map.insert(c, map.get(&c).copied().unwrap_or(0) + 1);
                    set.insert(c);
                }
            });
        });
        map.iter().for_each(|(k, v)| {
            if *v == 3 {
                priority_sum += char_to_priority(*k);
            }
        });
    });

    Ok(priority_sum.to_string())
}
