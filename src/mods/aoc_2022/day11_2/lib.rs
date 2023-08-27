struct Monkey {
    items: Vec<usize>,
    operation: Box<dyn Fn(usize) -> usize>,
    next_monkey_test: Box<dyn Fn(usize) -> usize>,
    item_count: usize,
}

fn simulate(monkeys: &mut Vec<Monkey>) {
    for monkey_idx in 0..monkeys.len() {
        monkeys[monkey_idx]
            .items
            .drain(..)
            .collect::<Vec<usize>>()
            .iter()
            .for_each(|item| {
                let new_item = (monkeys[monkey_idx].operation)(*item) % 9699690;
                let next_monkey_idx = (monkeys[monkey_idx].next_monkey_test)(new_item);
                monkeys[next_monkey_idx].items.push(new_item);
                monkeys[monkey_idx].item_count += 1;
            });
    }
}

pub fn run<T>(_: &str, _: T) -> Result<String, String>
where
    T: Iterator<Item = String>,
{
    let mut monkeys = vec![
        Monkey {
            item_count: 0,
            items: vec![91, 66],
            operation: Box::from(|old| old * 13),
            next_monkey_test: Box::from(|old| if old % 19 == 0 { 6 } else { 2 }),
        },
        Monkey {
            item_count: 0,
            items: vec![78, 97, 59],
            operation: Box::from(|old| old + 7),
            next_monkey_test: Box::from(|old| if old % 5 == 0 { 0 } else { 3 }),
        },
        Monkey {
            item_count: 0,
            items: vec![57, 59, 97, 84, 72, 83, 56, 76],
            operation: Box::from(|old| old + 6),
            next_monkey_test: Box::from(|old| if old % 11 == 0 { 5 } else { 7 }),
        },
        Monkey {
            item_count: 0,
            items: vec![81, 78, 70, 58, 84],
            operation: Box::from(|old| old + 5),
            next_monkey_test: Box::from(|old| if old % 17 == 0 { 6 } else { 0 }),
        },
        Monkey {
            item_count: 0,
            items: vec![60],
            operation: Box::from(|old| old + 8),
            next_monkey_test: Box::from(|old| if old % 7 == 0 { 1 } else { 3 }),
        },
        Monkey {
            item_count: 0,
            items: vec![57, 69, 63, 75, 62, 77, 72],
            operation: Box::from(|old| old * 5),
            next_monkey_test: Box::from(|old| if old % 13 == 0 { 7 } else { 4 }),
        },
        Monkey {
            item_count: 0,
            items: vec![73, 66, 86, 79, 98, 87],
            operation: Box::from(|old| old * old),
            next_monkey_test: Box::from(|old| if old % 3 == 0 { 5 } else { 2 }),
        },
        Monkey {
            item_count: 0,
            items: vec![95, 89, 63, 67],
            operation: Box::from(|old| old + 2),
            next_monkey_test: Box::from(|old| if old % 2 == 0 { 1 } else { 4 }),
        },
    ];
    for _ in 0..10000 {
        simulate(&mut monkeys);
    }

    let mut top_counts = monkeys
        .iter()
        .map(|monkey| monkey.item_count)
        .collect::<Vec<usize>>();
    top_counts.sort();
    top_counts.reverse();
    Ok((top_counts[0] * top_counts[1]).to_string())
}
