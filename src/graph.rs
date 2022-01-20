//! Simple graph implementation using adjacent list.

/// Specify the type used for the arc weight
pub type GraphWeight = u32;

use ndarray::Array2;

/// Graph's Arc. Define an arbitrary information A,
/// the destination of the arc and the weight of the arc.
pub struct Arc<A> {
    info: A,
    next: usize,
    weight: GraphWeight,
}

/// Graph implementation.
/// Store an information about each node in generic N
/// and the adjacent lists. Each arc has an associated information A.
pub struct Graph<N, A> {
    nodes: Vec<N>,
    adj: AdjacentList<A>,
}

impl<N, A> Graph<N, A> {
    /// Return the number of nodes in the graph.
    #[must_use]
    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    /// Return true if graph does not contain nodes.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.nodes.len() == 0
    }
}

/// A wrapper struct to contain the adjacent lists.
struct AdjacentList<A> {
    list: Vec<Vec<Arc<A>>>,
}

/// Convert the Graph from the adjacent list to the distance matrix representation.
/// Information in N and A will be lost; only the distance between two arcs is kept.
/// If there is not a direct connection between node i and j the distance is set to
/// ``GraphWeight::MAX``
    #[must_use]
    pub fn to_distance_matrix<N, A>(g: &Graph<N, A>) -> Array2<GraphWeight> {
    let mut dist_mat = Array2::from_elem((g.len(), g.len()), GraphWeight::MAX);
    for (i, adj) in g.adj.list.iter().enumerate() {
        dist_mat[(i, i)] = 0;
        for n in adj {
            dist_mat[(i, n.next)] = n.weight;
        }
    }
    dist_mat
}
