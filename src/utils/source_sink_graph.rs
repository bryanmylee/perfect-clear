use petgraph::graph::NodeIndex;
use petgraph::{Directed, Graph, Undirected};

pub struct SourceSinkGraph<N, E, Ty = Directed> {
    pub source: Option<NodeIndex>,
    pub sink: Option<NodeIndex>,
    pub graph: Graph<N, E, Ty>,
}

impl<N, E> SourceSinkGraph<N, E, Directed> {
    pub fn new() -> Self {
        SourceSinkGraph {
            source: None,
            sink: None,
            graph: Graph::new(),
        }
    }
}

impl<N, E> SourceSinkGraph<N, E, Undirected> {
    pub fn new_undirected() -> Self {
        SourceSinkGraph {
            source: None,
            sink: None,
            graph: Graph::new_undirected(),
        }
    }
}
