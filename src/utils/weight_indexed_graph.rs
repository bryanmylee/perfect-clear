use std::collections::HashMap;
use std::hash::Hash;

use petgraph::graph::NodeIndex;
use petgraph::{Directed, EdgeType, Graph, Undirected};

pub struct WeightIndexedGraph<N, E, Ty = Directed> {
    pub index_for_weight: HashMap<N, NodeIndex>,
    pub graph: Graph<N, E, Ty>,
}

impl<N, E> WeightIndexedGraph<N, E, Directed> {
    pub fn new() -> Self {
        WeightIndexedGraph {
            index_for_weight: HashMap::new(),
            graph: Graph::new(),
        }
    }
}

impl<N, E> WeightIndexedGraph<N, E, Undirected> {
    pub fn new_undirected() -> Self {
        WeightIndexedGraph {
            index_for_weight: HashMap::new(),
            graph: Graph::new_undirected(),
        }
    }
}

impl<N, E, Ty> WeightIndexedGraph<N, E, Ty>
where
    N: Hash + Eq + Copy,
    Ty: EdgeType,
{
    pub fn add_node(&mut self, weight: N) -> NodeIndex {
        if let Some(&index) = self.index_for_weight.get(&weight) {
            return index;
        }
        let index = self.graph.add_node(weight);
        self.index_for_weight.insert(weight, index);
        index
    }

    pub fn get_node_index(&self, weight: N) -> Option<NodeIndex> {
        self.index_for_weight.get(&weight).copied()
    }
}
