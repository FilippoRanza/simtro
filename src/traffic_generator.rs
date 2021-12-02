use rand;
use rand_distr::{Distribution, Poisson};
use splines::{Interpolation, Key, Spline};

type Node = f64;
type Int = u32;

const DEFAULT_NODE_VALUE: Node = 1.0;
const MINUTE_IN_HOUR: Int = 60;

pub trait TrafficGenerator {
    fn next_traffic_matrix(&self, step: Int) -> Int;
}

pub struct SimpleTrafficGenerator {
    spline: Spline<Node, Node>,
    scale: Node,
}

impl SimpleTrafficGenerator {
    pub fn new(conf: SimpleTrafficGeneratorConfig) -> Self {
        let steps = get_time_steps(&conf);
        let anchors = convert_anchor_vector(conf.anchors, conf.time_begin, conf.time_end, steps);
        let spline = spline_from_anchors(anchors);
        let scale = get_scale_value(&spline, steps, conf.traffic);
        Self { spline, scale }
    }

    fn get_passenger_probability_at(&self, step: Int) -> Node {
        self.spline.sample(step as Node).unwrap() * self.scale
    }
}

impl TrafficGenerator for SimpleTrafficGenerator {
    fn next_traffic_matrix(&self, step: Int) -> Int {
        let lambda = self.get_passenger_probability_at(step);
        let passengers = sample_poisson(lambda);
        passengers as Int
    }
}

fn get_scale_value(spline: &Spline<Node, Node>, end: Int, traffic: Int) -> Node {
    let magn = integrate_spline(spline, end);
    (traffic as Node) / magn
}

fn integrate_spline(spline: &Spline<Node, Node>, end: Int) -> Node {
    (0..end)
        .map(|i| i as Node)
        .map(|f| spline.sample(f).unwrap())
        .sum()
}

fn spline_from_anchors(anchors: Vec<(Node, Node)>) -> Spline<Node, Node> {
    let key_iter = anchors.into_iter().map(key_from_tuple).collect();
    Spline::from_vec(key_iter)
}

fn key_from_tuple<T, V>(t: (T, V)) -> Key<T, V> {
    Key::new(t.0, t.1, Interpolation::Cosine)
}

fn convert_anchor_vector(
    anchors: Vec<(Int, Node)>,
    time_begin: Int,
    time_end: Int,
    steps: Int,
) -> Vec<(Node, Node)> {
    let anchors = set_first(anchors, time_begin);
    let anchors = set_last(anchors, time_end);
    scale_vector(anchors, time_begin, time_end, steps)
}

fn scale_vector<T>(anchors: Vec<(Int, T)>, t0: Int, tf: Int, step_count: Int) -> Vec<(Node, T)> {
    let scale = (step_count as Node) / ((tf - t0) as Node);
    anchors
        .into_iter()
        .map(|(t, p)| (t - t0, p))
        .map(|(t, p)| (t as Node, p))
        .map(|(t, p)| (t * scale, p))
        .collect()
}

fn set_first(anchors: Vec<(Int, Node)>, time_begin: Int) -> Vec<(Int, Node)> {
    set_value_at_index_if_time_missing(anchors, time_begin, |v, t| v.insert(0, t), |v| v.first())
}

fn set_last(anchors: Vec<(Int, Node)>, time_end: Int) -> Vec<(Int, Node)> {
    set_value_at_index_if_time_missing(anchors, time_end, |v, t| v.push(t), |v| v.last())
}

fn set_value_at_index_if_time_missing<F, G>(
    mut anchors: Vec<(Int, Node)>,
    time: Int,
    f: F,
    g: G,
) -> Vec<(Int, Node)>
where
    F: Fn(&mut Vec<(Int, Node)>, (Int, Node)),
    G: Fn(&Vec<(Int, Node)>) -> Option<&'_ (Int, Node)>,
{
    if let Some((t, _)) = g(&anchors) {
        if *t != time {
            f(&mut anchors, default_anchor(time))
        }
        anchors
    } else {
        vec![default_anchor(time)]
    }
}

fn get_time_steps(conf: &SimpleTrafficGeneratorConfig) -> Int {
    (conf.time_end - conf.time_begin) * conf.minute_resolution * MINUTE_IN_HOUR
}

fn default_anchor(time: Int) -> (Int, Node) {
    (time, DEFAULT_NODE_VALUE)
}

fn sample_poisson(lambda: Node) -> Node {
    let poi = Poisson::new(lambda).unwrap();
    poi.sample(&mut rand::thread_rng())
}

pub struct SimpleTrafficGeneratorConfig {
    pub anchors: Vec<(Int, Node)>,
    pub time_begin: Int,
    pub time_end: Int,
    pub minute_resolution: Int,
    pub traffic: Int,
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_value_insertion() {
        let anchors = vec![(5, 5.6)];
        let anchors = convert_anchor_vector(anchors, 4, 6, 3);
        let correct = vec![(0., 1.0), (1.5, 5.6), (3., 1.0)];
        assert_eq!(anchors, correct);
    }

    #[test]
    fn test_non_required_value_add() {
        let anchors = vec![(6, 1.), (7, 10.), (12, 4.), (15, 2.), (18, 6.), (20, 1.)];
        let count = anchors.len();
        let result = convert_anchor_vector(anchors, 6, 20, 6);
        assert_eq!(result.len(), count);
    }

    #[test]
    fn test_probability_generator() {
        let anchors = vec![(6, 1.), (7, 10.), (12, 4.), (15, 2.), (18, 6.), (20, 1.)];
        let traffic = 1450;
        let conf = SimpleTrafficGeneratorConfig {
            anchors,
            time_begin: 5,
            time_end: 21,
            minute_resolution: 2,
            traffic,
        };
        let steps = get_time_steps(&conf);
        let stg = SimpleTrafficGenerator::new(conf);
        let res: Node = (0..steps)
            .map(|i| stg.get_passenger_probability_at(i))
            .sum();
        let err = (res - (traffic as Node)).abs();
        assert!(err < 1e-8);
    }
}
