mod simple_traffic_generator;
mod simple_traffic_generator_factory;
mod traffic_generator;

pub use simple_traffic_generator::SimpleTrafficGenerator;
pub use traffic_generator::TrafficGenerator;

pub use simple_traffic_generator_factory::{simple_traffic_generator_factory, SimpleTrafficGeneratorConfig};

/// Allow to change probability type in no-time
type Node = f64;
/// Allow to change step type in no-time
type Int = u32;
