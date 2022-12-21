use petgraph::graph::NodeIndex;
use petgraph::{Directed, EdgeType, Graph, Undirected};

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

impl<N, E, Ty> SourceSinkGraph<N, E, Ty>
where
    Ty: EdgeType,
{
    pub fn add_source_node(&mut self, weight: N) -> NodeIndex {
        let idx = self.graph.add_node(weight);
        self.source = Some(idx);
        idx
    }

    pub fn add_sink_node(&mut self, weight: N) -> NodeIndex {
        let idx = self.graph.add_node(weight);
        self.sink = Some(idx);
        idx
    }
}
