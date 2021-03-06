//! Handle cars fleet on the network.
//! This module is used to manage
//! trains running on the network.

use crate::car;

pub struct Fleet<T> {
    running: Vec<car::Car<T>>,
}

impl<T> Fleet<T> {
    #[must_use]
    pub fn new(fleet_size: usize) -> Self {
        let running = Vec::with_capacity(fleet_size);
        Self { running }
    }

    /// Iterate through all possible running trains
    pub fn running_cars_iter(&mut self) -> impl Iterator<Item = &mut car::Car<T>> {
        self.running.iter_mut()
    }

    /// Iterate through all trains currently in a station
    pub fn in_station_car_iter(&mut self) -> impl Iterator<Item = &mut car::Car<T>> {
        self.running_cars_iter().filter(|c| c.in_station())
    }

    /// Add a new train to list of running trains.
    pub fn start_train(&mut self, car: car::Car<T>) {
        self.running.push(car);
    }

    pub fn len(&self) -> usize {
        self.running.len()
    }

    pub fn is_empty(&self) -> bool {
        self.running.is_empty()
    }
}
