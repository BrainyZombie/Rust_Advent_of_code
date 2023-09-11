pub mod floyd_warshall;

use std::{borrow::Borrow, hash::Hash};

#[derive(Debug, Clone, Copy)]
pub struct Edge<ID> {
    pub path_weight: isize,
    pub id: ID,
}
pub trait GraphNode<N>
where
    Self::NodeId: Borrow<N>,
    N: Hash + Eq + ?Sized,
{
    type NodeId: ?Sized + Hash + Eq;
    fn get_id(&self) -> &N;
    fn get_edges(&self) -> Vec<Edge<&N>>;
}

pub trait Graph<N>
where
    N: Hash + Eq + ?Sized,
{
    type GraphNode: GraphNode<N>;
    fn get_nodes(&self) -> Vec<&Self::GraphNode>;
    fn get_node(&self, id: &N) -> Option<&Self::GraphNode>;
}
