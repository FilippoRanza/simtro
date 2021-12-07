//! Handle cars fleet on the network.
//! This module is mainly responsable of two things.
//! 1. Manage trains currently running on the line
//! 2. Manage trains currently at deposit.
//! A train is said to be enabled when from the deposit is set on
//! a running state and it is said to be disabled when from the running state
//! it set into deposit.

use crate::car;

struct Fleet {
    running: Vec<car::Car>,
    deposit: Vec<car::Car>,
    speed: usize,
}

impl Fleet {
    fn new(deposit: Vec<car::Car>, speed: usize) -> Self {
        let running = Vec::with_capacity(deposit.len());
        Self {
            deposit,
            running,
            speed,
        }
    }

    fn move_cars(&mut self) {
        for car in self.running.iter_mut() {
            car.move_car(self.speed);
        }
    }

    fn start_trains(&mut self) {
        todo! {}
    }
}
