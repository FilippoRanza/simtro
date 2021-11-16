use ndarray;

pub trait TrafficGenerator {
    fn next_traffic_matrix<'a>(&'a self, step: u32) -> &'a ndarray::Array2<u32>;
}

pub struct SimpleTrafficGenerator {}

impl TrafficGenerator for SimpleTrafficGenerator {
    fn next_traffic_matrix<'a>(&'a self, step: u32) -> &'a ndarray::Array2<u32> {
        todo! {}
    }
}
