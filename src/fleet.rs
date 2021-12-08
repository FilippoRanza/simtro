//! Handle cars fleet on the network.
//! This module is mainly responsable of two things.
//! 1. Manage trains currently running on the line
//! 2. Manage trains currently at deposit.
//! A train is said to be enabled when from the deposit is set on
//! a running state and it is said to be disabled when from the running state
//! it set into deposit.

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

    pub fn running_cars_iter(&mut self) -> impl Iterator<Item = &mut car::Car> {
        self.running.iter_mut()
    }

    pub fn in_station_car_iter(&mut self) -> impl Iterator<Item = &mut car::Car> {
        self.running_cars_iter().filter(|c| c.in_station())
    }

    pub fn start_train(&mut self, car: car::Car) {
        self.running.push(car)
    }
}
