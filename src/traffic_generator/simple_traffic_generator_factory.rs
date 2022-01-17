//! This module implements a factory to build a SimpleTrafficGenerator
//! matrix that is required by a PassengerGeneraror.

use super::SimpleTrafficGenerator;
use super::{Int, Node};

pub struct TrafficConfig {
    src: usize,
    dst: usize,
    anchors: Vec<(Int, Node)>,
    traffic: Int,
}

impl TrafficConfig {
    pub fn new(dir: (usize, usize), traffic: Int, anchors: Vec<(Int, Node)>) -> Self {
        Self {
            traffic,
            anchors,
            src: dir.0,
            dst: dir.1,
        }
    }
}

pub fn factory(
    begin: Int,
    end: Int,
    res: Int,
    traffic_configs: Vec<TrafficConfig>,
    default: Option<Vec<(Int, Node)>>,
) -> Vec<Vec<SimpleTrafficGenerator>> {
    todo! {}
}

pub struct FactoryConfig {}

pub fn make_factory_cofig(
    begin: Int,
    end: Int,
    res: Int,
    traffic_configs: Vec<TrafficConfig>,
) -> FactoryConfig {
    todo! {}
}
