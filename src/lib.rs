pub mod car;
pub mod engine;
pub mod fleet;
pub mod graph;
pub mod line;
pub mod passenger;
pub mod station;
pub mod traffic_generator;

pub mod routes;
pub mod utils;

/// Allow to change type in no-time
type Node = f64;
type Int = u32;

const MINUTE_IN_HOUR: Int = 60;

#[must_use]
pub fn get_steps(begin: Int, end: Int, minute_resolution: Int) -> Int {
    (end - begin) * MINUTE_IN_HOUR * minute_resolution
}
