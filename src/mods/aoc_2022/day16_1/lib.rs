use std::collections::HashMap;

#[derive(Debug)]
struct Valve<'a> {
    id: &'a str,
    is_open: bool,
    flow_rate: usize,
    next_valves: Vec<&'a str>,
}

fn create_valves<'a>(input: &'a str) -> Vec<Valve<'a>> {
    let valves: Vec<Valve<'a>> = input
        .lines()
        .map(|line| Valve {
            id: &line[6..=7],
            is_open: false,
            flow_rate: line[23..line.find(';').unwrap()].parse().unwrap(),
            next_valves: line[line
                .find("valves")
                .unwrap_or_else(|| line.find("valve").unwrap() - 1)
                + 7..]
                .split(", ")
                .collect(),
        })
        .collect();

    valves
}

fn get_max_possible_flow(valves: &mut HashMap<String, Valve>, time_remaining: usize) -> usize {
    let mut ordered_open_valve_flows: Vec<usize> = valves
        .iter()
        .flat_map(|v| {
            if !v.1.is_open {
                Some(v.1.flow_rate)
            } else {
                None
            }
        })
        .collect();
    ordered_open_valve_flows.sort_unstable();

    ordered_open_valve_flows
        .iter()
        .rev()
        .fold(
            (0, time_remaining),
            |(existing_flow, time_remaining), flow| {
                if time_remaining < 2 {
                    (existing_flow, 0)
                } else {
                    (
                        existing_flow + flow * (time_remaining - 1),
                        time_remaining - 2,
                    )
                }
            },
        )
        .0
}

fn dfs(
    valves: &mut HashMap<String, Valve>,
    current_valve_key: &str,
    time_remaining: usize,
    needed_flow: usize,
) -> usize {
    if time_remaining < 2 {
        return 0;
    }
    let max_possible_flow: usize = get_max_possible_flow(valves, time_remaining);
    if needed_flow >= max_possible_flow {
        return 0;
    }

    let current_valve = valves.get_mut(current_valve_key).unwrap();

    let mut result = 0usize;
    let next_valves = current_valve.next_valves.clone();

    if !current_valve.is_open && current_valve.flow_rate > 0 {
        current_valve.is_open = true;
        let current_valve_total_flow = current_valve.flow_rate * (time_remaining - 1);
        next_valves.iter().for_each(|next_valve_key| {
            result = result.max(dfs(
                valves,
                next_valve_key,
                time_remaining - 2,
                result.max(needed_flow.saturating_sub(current_valve_total_flow)),
            ));
        });
        result += current_valve_total_flow;
        valves.get_mut(current_valve_key).unwrap().is_open = false;
    }

    next_valves.iter().for_each(|next_valve| {
        result = result.max(dfs(
            valves,
            next_valve,
            time_remaining - 1,
            result.max(needed_flow),
        ));
    });

    result
}

pub fn run<T>(input: &str, _: T) -> Result<String, String>
where
    T: Iterator<Item = String>,
{
    let mut valves: HashMap<String, Valve> = HashMap::from_iter(
        create_valves(input)
            .drain(..)
            .map(|valve| (valve.id.to_string(), valve)),
    );
    let result = dfs(&mut valves, "AA", 30, 0);
    Ok(result.to_string())
}
