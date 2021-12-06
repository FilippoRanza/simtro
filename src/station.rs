use crate::passenger::Passenger;

pub trait StationClass {
    fn is_terminus(&self) -> bool;
    fn is_interchange(&self) -> bool;
}

/// Define general purpose method for a passenger
/// station. Trait main usage is to improve 
/// client code generality.
pub trait PassengerStation: Send + Sync {
    
    /// Add passengers to the station implementation
    fn enter_passenger(&mut self, p: Passenger);
}

pub struct Station {
    state: StationState,
    passengers: Vec<Passenger>,
}

impl Station {
    pub fn passengers_to(&mut self, dst: usize) {}
}

impl PassengerStation for Station {
    fn enter_passenger(&mut self, p: Passenger) {
        self.passengers.push(p)
    }
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
