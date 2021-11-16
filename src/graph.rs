pub struct Arc<A> {
    pub info: A,
    pub next: usize,
}

pub struct Graph<N, A> {
    nodes: Vec<N>,
    adj: AdjacentList<A>,
}

struct AdjacentList<A> {
    list: Vec<Vec<Arc<A>>>,
}
