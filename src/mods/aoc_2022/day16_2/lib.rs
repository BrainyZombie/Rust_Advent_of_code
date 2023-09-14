use std::{collections::HashMap, rc::Rc};

use crate::mods::aoc_helpers::graph::{floyd_warshall::floyd_warshall, Edge, Graph, GraphNode};

#[derive(Debug)]
struct Valve {
    id: Rc<str>,
    flow_rate: usize,
    next_valves: Vec<Rc<str>>,
}

impl GraphNode<str> for Valve {
    type NodeId = Rc<str>;
    fn get_id(&self) -> &str {
        &self.id[..]
    }
    fn get_edges(&self) -> Vec<Edge<&str>> {
        self.next_valves
            .iter()
            .map(|n| Edge::<&str> {
                id: n,
                path_weight: 1,
            })
            .collect()
    }
}

type ValveGraph = HashMap<String, Valve>;

impl Graph<str> for ValveGraph {
    type GraphNode = Valve;
    fn get_node(&self, id: &str) -> Option<&Self::GraphNode> {
        self.get(id)
    }
    fn get_nodes(&self) -> Vec<&Self::GraphNode> {
        self.values().collect()
    }
}

fn create_valves(input: &str) -> Vec<Valve> {
    let valves: Vec<Valve> = input
        .lines()
        .map(|line| Valve {
            id: line[6..=7].into(),
            flow_rate: line[23..line.find(';').unwrap()].parse().unwrap(),
            next_valves: line[line
                .find("valves")
                .unwrap_or_else(|| line.find("valve").unwrap() - 1)
                + 7..]
                .split(", ")
                .map(|key| key.into())
                .collect(),
        })
        .collect();

    valves
}

fn get_max_possible_flow(
    valves: &ValveGraph,
    open_valves: &[&str],
    mut time_remaining: usize,
) -> usize {
    open_valves.iter().fold(0, |acc, v| {
        if time_remaining > 2 {
            time_remaining -= 2;
            return acc + valves.get(*v).unwrap().flow_rate * (time_remaining + 1);
        }
        acc
    })
}

fn dfs(
    current_valve: &str,
    valves: &ValveGraph,
    shortest_paths: &impl Fn(&str, &str) -> Option<isize>,
    open_valves: &[&str],
    time_remaining: usize,
    needed_flow: usize,
) -> usize {
    if time_remaining < 2 {
        return 0;
    }

    let current_valve_pressure =
        valves.get(current_valve).unwrap().flow_rate * (time_remaining - 1);

    if current_valve_pressure + get_max_possible_flow(valves, open_valves, time_remaining - 1)
        < needed_flow
    {
        return 0;
    }

    let mut result: usize = 0;
    open_valves.iter().for_each(|to_open| {
        let path = (shortest_paths)(current_valve, to_open);
        if let Some(distance) = path {
            let open_valves: Vec<&str> = open_valves
                .iter()
                .filter(|v| *v != to_open)
                .copied()
                .collect();
            result = result.max(dfs(
                to_open,
                valves,
                shortest_paths,
                &open_valves,
                time_remaining.saturating_sub(distance as usize + 1),
                result,
            ));
        }
    });
    result + current_valve_pressure
}

pub fn run<T>(input: &str, _: T) -> Result<String, String>
where
    T: Iterator<Item = String>,
{
    let mut valves = create_valves(input);

    let valve_graph: ValveGraph =
        HashMap::from_iter(valves.drain(..).map(|valve| (valve.id.to_string(), valve)));

    let shortest_paths = floyd_warshall(&valve_graph);
    let open_valves: Vec<&str> = valve_graph
        .iter()
        .filter_map(|(key, valve)| {
            if valve.flow_rate != 0 {
                Some(&key[..])
            } else {
                None
            }
        })
        .collect();

    let result = dfs("AA", &valve_graph, &shortest_paths, &open_valves, 31, 0);
    Ok(result.to_string())
}
