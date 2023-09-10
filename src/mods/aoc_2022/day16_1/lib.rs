use std::collections::HashMap;

use crate::mods::aoc_helpers::graph::{
    floyd_warshall::floyd_warshall, Graph, GraphNode, NodeId, NodeNeighbor,
};

impl NodeId for String {}
#[derive(Debug)]
struct Valve {
    id: String,
    flow_rate: usize,
    next_valves: Vec<NodeNeighbor<String>>,
}

impl GraphNode for Valve {
    type NodeId = String;
    fn get_id(&self) -> &Self::NodeId {
        &self.id
    }
    fn get_neighbors(&self) -> Vec<NodeNeighbor<Self::NodeId>> {
        self.next_valves.clone()
    }
}

type ValveGraph = HashMap<String, Valve>;

impl Graph for ValveGraph {
    type GraphNode = Valve;
    fn get_node(&self, id: <Self::GraphNode as GraphNode>::NodeId) -> Option<&Self::GraphNode> {
        self.get(&id)
    }
    fn get_nodes(&self) -> Vec<&Self::GraphNode> {
        self.values().collect()
    }
}

fn create_valves(input: &str) -> Vec<Valve> {
    let valves: Vec<Valve> = input
        .lines()
        .map(|line| Valve {
            id: line[6..=7].to_string(),
            flow_rate: line[23..line.find(';').unwrap()].parse().unwrap(),
            next_valves: line[line
                .find("valves")
                .unwrap_or_else(|| line.find("valve").unwrap() - 1)
                + 7..]
                .split(", ")
                .map(|key| NodeNeighbor {
                    id: key.to_string(),
                    path_weight: 1,
                })
                .collect(),
        })
        .collect();

    valves
}

fn dfs(
    current_valve: &str,
    valves: &ValveGraph,
    shortest_paths: &impl Fn(&String, &String) -> Option<isize>,
    open_valves: &[&str],
    time_remaining: usize,
) -> isize {
    if time_remaining < 2 {
        return 0;
    }
    let mut v: Vec<&Valve> = open_valves
        .iter()
        .map(|v| valves.get(*v).unwrap())
        .collect();
    v.sort_by(|v1, v2| v2.flow_rate.cmp(&v1.flow_rate));

    let current_valve_pressure =
        ((valves.get(current_valve).unwrap().flow_rate) * (time_remaining - 1)) as isize;
    let mut result: isize = 0;
    open_valves.iter().for_each(|to_open| {
        let path = (shortest_paths)(&current_valve.to_string(), &to_open.to_string());
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
    let result = dfs("AA", &valve_graph, &shortest_paths, &open_valves, 31);
    Ok(result.to_string())
}
