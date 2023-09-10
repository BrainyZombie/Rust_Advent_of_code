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
            if !v.1.is_open && v.1.flow_rate > 0 {
                Some(v.1.flow_rate)
            } else {
                None
            }
        })
        .collect();
    ordered_open_valve_flows.sort_unstable();
    ordered_open_valve_flows.reverse();

    ordered_open_valve_flows
        .chunks(2)
        .fold(
            (0, time_remaining),
            |(existing_flow, time_remaining), flow| {
                if time_remaining < 2 {
                    (existing_flow, 0)
                } else {
                    (
                        existing_flow
                            + (flow[0] + flow.get(1).unwrap_or(&0)) * (time_remaining - 1),
                        time_remaining - 2,
                    )
                }
            },
        )
        .0
}

fn g_v<'a, 'b>(valves: &'a mut HashMap<String, Valve<'b>>, key: &str) -> &'a mut Valve<'b> {
    valves.get_mut(key).unwrap()
}

fn dfs(
    valves: &mut HashMap<String, Valve>,
    current_valve_key: (&str, &str),
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

    let mut final_result = 0usize;
    let next_valves = (
        g_v(valves, current_valve_key.0).next_valves.clone(),
        g_v(valves, current_valve_key.1).next_valves.clone(),
    );

    if !g_v(valves, current_valve_key.0).is_open
        && g_v(valves, current_valve_key.0).flow_rate > 0
        && !g_v(valves, current_valve_key.1).is_open
        && g_v(valves, current_valve_key.1).flow_rate > 0
        && current_valve_key.0 != current_valve_key.1
    {
        g_v(valves, current_valve_key.0).is_open = true;
        g_v(valves, current_valve_key.1).is_open = true;

        let current_valve_total_flow = (g_v(valves, current_valve_key.0).flow_rate
            + g_v(valves, current_valve_key.1).flow_rate)
            * (time_remaining - 1);

        let mut result = 0usize;

        next_valves
            .0
            .iter()
            .flat_map(|y| next_valves.1.iter().clone().map(move |x| (*x, *y)))
            .for_each(|next_valve_key| {
                result = result.max(dfs(
                    valves,
                    next_valve_key,
                    time_remaining - 2,
                    final_result
                        .max(result.max(needed_flow.saturating_sub(current_valve_total_flow))),
                ));
            });
        result += current_valve_total_flow;
        final_result = final_result.max(result);
        g_v(valves, current_valve_key.0).is_open = false;
        g_v(valves, current_valve_key.1).is_open = false;
    }

    if !g_v(valves, current_valve_key.0).is_open && g_v(valves, current_valve_key.0).flow_rate > 0 {
        g_v(valves, current_valve_key.0).is_open = true;

        let current_valve_total_flow =
            g_v(valves, current_valve_key.0).flow_rate * (time_remaining - 1);

        let mut result = 0usize;

        next_valves.1.iter().for_each(|next_valve_key| {
            result = result.max(dfs(
                valves,
                (current_valve_key.0, next_valve_key),
                time_remaining - 1,
                final_result.max(result.max(needed_flow.saturating_sub(current_valve_total_flow))),
            ));
        });
        result += current_valve_total_flow;
        final_result = final_result.max(result);
        g_v(valves, current_valve_key.0).is_open = false;
    }

    if !g_v(valves, current_valve_key.1).is_open && g_v(valves, current_valve_key.1).flow_rate > 0 {
        g_v(valves, current_valve_key.1).is_open = true;

        let current_valve_total_flow =
            g_v(valves, current_valve_key.1).flow_rate * (time_remaining - 1);

        let mut result = 0usize;

        next_valves.1.iter().for_each(|next_valve_key| {
            result = result.max(dfs(
                valves,
                (next_valve_key, current_valve_key.1),
                time_remaining - 1,
                final_result.max(result.max(needed_flow.saturating_sub(current_valve_total_flow))),
            ));
        });
        result += current_valve_total_flow;
        final_result = final_result.max(result);
        g_v(valves, current_valve_key.1).is_open = false;
    }

    next_valves
        .0
        .iter()
        .flat_map(|y| next_valves.1.iter().clone().map(move |x| (*x, *y)))
        .for_each(|next_valve_key| {
            final_result = final_result.max(dfs(
                valves,
                next_valve_key,
                time_remaining - 1,
                final_result.max(needed_flow),
            ));
        });

    final_result
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
    let result = dfs(&mut valves, ("AA", "AA"), 26, 0);
    Ok(result.to_string())
}
