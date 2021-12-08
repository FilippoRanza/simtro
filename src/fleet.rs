//! Handle cars fleet on the network.
//! This module is used to manage
//! trains running on the network.

use crate::car;

pub struct Fleet {
    running: Vec<car::Car>,
    speed: usize,
}

impl Fleet {
    fn new(fleet_size: usize, speed: usize) -> Self {
        let running = Vec::with_capacity(fleet_size);
        Self { running, speed }
    }

    /// Iterate through all possible running trains
    pub fn running_cars_iter(&mut self) -> impl Iterator<Item = &mut car::Car> {
        self.running.iter_mut()
    }

    /// Iterate through all trains currently in a station
    pub fn in_station_car_iter(&mut self) -> impl Iterator<Item = &mut car::Car> {
        self.running_cars_iter().filter(|c| c.in_station())
    }

    /// Add a new train to list of running trains.
    pub fn start_train(&mut self, car: car::Car) {
        self.running.push(car)
    }
}
