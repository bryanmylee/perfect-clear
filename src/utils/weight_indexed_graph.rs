use petgraph::graph::{EdgeIndex, NodeIndex};
use petgraph::{Directed, EdgeType, Graph, Undirected};
use std::collections::HashMap;
use std::hash::Hash;

pub struct WeightIndexedGraph<N, E, Ty = Directed> {
    pub graph: Graph<N, E, Ty>,
    pub index_for_weight: HashMap<N, NodeIndex>,
}

impl<N, E> WeightIndexedGraph<N, E, Directed> {
    pub fn new() -> Self {
        WeightIndexedGraph {
            graph: Graph::new(),
            index_for_weight: HashMap::new(),
        }
    }
}

impl<N, E> WeightIndexedGraph<N, E, Undirected> {
    pub fn new_undirected() -> Self {
        WeightIndexedGraph {
            graph: Graph::new_undirected(),
            index_for_weight: HashMap::new(),
        }
    }
}

impl<N, E, Ty> WeightIndexedGraph<N, E, Ty>
where
    N: Hash + Eq + Copy,
    Ty: EdgeType,
{
    pub fn add_node(&mut self, weight: N) -> NodeIndex {
        if self.index_for_weight.contains_key(&weight) {
            return self.index_for_weight[&weight];
        }
        let idx = self.graph.add_node(weight);
        self.index_for_weight.insert(weight, idx);
        idx
    }

    pub fn add_edge(&mut self, a: NodeIndex, b: NodeIndex, weight: E) -> EdgeIndex {
        self.graph.add_edge(a, b, weight)
    }
}
