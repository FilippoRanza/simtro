use crate::passenger::Passenger;

pub struct Station {
    state: StationState,
    passengers: Vec<Passenger>,
}

impl Station {
    pub fn enter_passenger(&mut self, p: Passenger) {
        self.passengers.push(p)
    }
}


pub enum StationState {
    Free,
    Car(usize)
}


pub struct Railway {
    length: usize
}
