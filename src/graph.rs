pub type GraphWeight = u32;

use crate::utils::matrix_utils;
use ndarray::Array2;

pub struct Arc<A> {
    info: A,
    next: usize,
    weight: GraphWeight,
}

pub struct Graph<N, A> {
    nodes: Vec<N>,
    adj: AdjacentList<A>,
}

impl<N, A> Graph<N, A> {
    pub fn len(&self) -> usize {
        self.nodes.len()
    }
}

struct AdjacentList<A> {
    list: Vec<Vec<Arc<A>>>,
}

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
