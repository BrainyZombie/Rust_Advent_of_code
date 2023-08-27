struct Monkey {
    items: Vec<usize>,
    operation: Box<dyn Fn(usize) -> usize>,
    next_monkey_test: Box<dyn Fn(usize) -> usize>,
    item_count: usize,
}

fn create_monkeys(input: &str) -> Vec<Monkey> {
    let lines = input.lines().peekable();

    struct TestTransients {
        num: Option<usize>,
        true_monkey: Option<usize>,
        false_monkey: Option<usize>,
    }

    let mut operation_buffer: Option<Box<dyn Fn(usize) -> usize>> = None;
    let mut items_buffer: Vec<usize> = vec![];
    let mut current_monkey_idx = 0;
    let mut next_monkey_test: TestTransients = TestTransients {
        num: None,
        true_monkey: None,
        false_monkey: None,
    };

    lines.fold(vec![], |mut acc, line| {
        let tokens: Vec<&str> = line.trim().split(' ').collect();

        match tokens[0] {
            "Monkey" => {
                current_monkey_idx = tokens[1][0..tokens[1].len() - 1].parse().unwrap();
            }
            "Starting" => {
                tokens[2..].iter().for_each(|token| {
                    items_buffer.push(token.split(",").next().unwrap().parse().unwrap())
                });
            }
            "Operation:" => {
                operation_buffer = Some(match tokens[5] {
                    "old" => Box::from(|old| old * old),
                    _ => {
                        let val: usize = tokens[5].parse().unwrap();
                        match tokens[4] {
                            "+" => Box::from(move |old| old + val),
                            "*" => Box::from(move |old| old * val),
                            _ => panic!(),
                        }
                    }
                });
            }
            "Test:" => {
                next_monkey_test.num = Some(tokens[3].parse().unwrap());
            }
            "If" => {
                if matches!(tokens[1], "true:") {
                    next_monkey_test.true_monkey = Some(tokens[5].parse().unwrap());
                } else {
                    next_monkey_test.false_monkey = Some(tokens[5].parse().unwrap());
                    if let TestTransients {
                        true_monkey: Some(true_monkey),
                        num: Some(num),
                        false_monkey: Some(false_monkey),
                    } = next_monkey_test
                    {
                        acc.push(Monkey {
                            item_count: 0,
                            items: Vec::from_iter(items_buffer.drain(..)),
                            operation: operation_buffer.take().unwrap(),
                            next_monkey_test: Box::from(move |worry| {
                                if worry % num == 0 {
                                    true_monkey
                                } else {
                                    false_monkey
                                }
                            }),
                        });
                    } else {
                        panic!();
                    }
                }
            }
            _ => {}
        }
        acc
    })
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

pub fn run<T>(input: &str, _: T) -> Result<String, String>
where
    T: Iterator<Item = String>,
{
    let mut monkeys = create_monkeys(input);
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
