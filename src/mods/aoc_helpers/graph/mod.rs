pub mod floyd_warshall;

use std::hash::Hash;

pub trait NodeId: Eq + Hash + Clone {}

#[derive(Debug, Clone, Copy)]
pub struct NodeNeighbor<ID: NodeId> {
    pub path_weight: isize,
    pub id: ID,
}
pub trait GraphNode {
    type NodeId: NodeId;
    fn get_id(&self) -> &Self::NodeId;
    fn get_neighbors(&self) -> Vec<NodeNeighbor<Self::NodeId>>;
}

pub trait Graph {
    type GraphNode: GraphNode;
    fn get_nodes(&self) -> Vec<&Self::GraphNode>;
    fn get_node(&self, id: <Self::GraphNode as GraphNode>::NodeId) -> Option<&Self::GraphNode>;
}
