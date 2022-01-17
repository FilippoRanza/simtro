//! This module define the TrafficGenerator trait

use super::Int;

/// A type implementing this trait can
/// be used to generate the step by step
/// traffic from one station to a specific
/// destination.
pub trait TrafficGenerator: Send + Sync {
    /// Return number of passenger arriving at the station
    /// at request step. This are passengers going to a
    /// specific destination.
    fn next_traffic_flow(&self, step: Int) -> Int;
}
