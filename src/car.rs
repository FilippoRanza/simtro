use crate::passenger::Passenger;

pub struct Car {
    passengers: Vec<Passenger>,
    state: CarState,
    destination: usize,
}

pub enum CarState {
    Stop(usize),
    Rail(usize),
}
