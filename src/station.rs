use crate::passenger::Passenger;

pub trait StationClass {
    fn is_terminus(&self) -> bool;
    fn is_interchange(&self) -> bool;
}

pub struct Station {
    state: StationState,
    passengers: Vec<Passenger>,
}

impl Station {
    pub fn enter_passenger(&mut self, p: Passenger) {
        self.passengers.push(p)
    }

    pub fn passengers_to(&mut self, dst: usize) {}
}

pub enum StationState {
    Free,
    Car(usize),
}

pub struct Railway {
    length: usize,
}

pub struct RailwayLine {
    terminus_a: usize,
    terminus_b: usize,
}
