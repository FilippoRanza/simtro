use crate::passenger::Passenger;

pub struct Car {
    passengers: Vec<Passenger>,
    state: CarState,
    destination: usize,
}

impl Car {
    pub fn unboard_passengers(&mut self) {
        todo! {}
    }

    pub fn board_passengers(&mut self, ps: &mut Vec<Passenger>) {
        self.passengers.append(ps);
    }

    pub fn in_station(&self) -> bool {
        matches! {self.state, CarState::Stop(_)}
    }

    pub fn at_station(&self, s: usize) -> bool {
        matches! {self.state, CarState::Stop(x) if s == x}
    }

    pub fn get_destination(&self) -> usize {
        self.destination
    }

    pub fn move_car(&mut self, speed: usize) {
        self.state = update_state(&self.state, speed, self.destination);
    }
}

pub enum CarState {
    Stop(usize),
    Rail(usize),
}

fn update_state(state: &CarState, speed: usize, destination: usize) -> CarState {
    match state {
        CarState::Rail(s) => {
            if *s < speed {
                CarState::Stop(destination)
            } else {
                CarState::Rail(s - speed)
            }
        }
        CarState::Stop(x) => CarState::Stop(*x),
    }
}
