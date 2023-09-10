use std::collections::HashMap;

use super::{Graph, GraphNode, NodeNeighbor};

pub fn floyd_warshall<G: Graph>(
    graph: &G,
) -> impl Fn(
    &<<G as Graph>::GraphNode as GraphNode>::NodeId,
    &<<G as Graph>::GraphNode as GraphNode>::NodeId,
) -> Option<isize> {
    let mut shortest_paths = HashMap::new();

    let nodes = graph.get_nodes();
    let node_ids: Vec<_> = nodes.iter().map(|node| node.get_id()).collect();

    nodes.iter().for_each(|from_node| {
        let mut paths = HashMap::new();

        from_node.get_neighbors().iter().for_each(
            |NodeNeighbor {
                 id: to_node,
                 path_weight: weight,
             }| {
                paths.insert(to_node.clone(), *weight);
            },
        );
        shortest_paths.insert(from_node.get_id().clone(), paths);
    });

    node_ids.iter().for_each(|middle_node_id| {
        node_ids.iter().for_each(|from_node_id| {
            node_ids.iter().for_each(|to_node_id| {
                if !(from_node_id == to_node_id
                    || from_node_id == middle_node_id
                    || to_node_id == middle_node_id)
                {
                    let path_len = shortest_paths
                        .get(from_node_id)
                        .and_then(|from_distances| from_distances.get(to_node_id).copied());

                    let half_path_1_len = shortest_paths
                        .get(from_node_id)
                        .and_then(|from_distances| from_distances.get(middle_node_id).copied());

                    let half_path_2_len = shortest_paths
                        .get(middle_node_id)
                        .and_then(|from_distances| from_distances.get(to_node_id).copied());

                    let middle_path_len =
                        half_path_1_len.and_then(|p1| half_path_2_len.map(|p2| p1 + p2));

                    let final_path_len = path_len
                        .and_then(|p1| middle_path_len.map(|p2| p1.min(p2)).or(Some(p1)))
                        .or(middle_path_len);

                    if let Some(len) = final_path_len {
                        let from_distances = shortest_paths.get_mut(from_node_id).unwrap();

                        from_distances.insert((*to_node_id).clone(), len);
                    }
                }
            })
        })
    });

    move |from, to| {
        shortest_paths
            .get(from)
            .and_then(|from_distances| from_distances.get(to))
            .copied()
    }
}
