use crate::car::Car;
use crate::passenger::Passenger;
use crate::routes::{MetroDirection, MetroInterchange};
use crate::utils::index_list;

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

pub trait BoardPassengers: Send + Sync {
    fn board_passengers(&mut self, c: &mut Car);
}

pub struct Station<'a> {
    state: StationState,
    index: usize,
    direction: &'a MetroDirection,
    interchange: &'a MetroInterchange,
    passengers: index_list::IndexList<Passenger>,
}

impl<'a> Station<'a> {
    pub fn passengers_to(&mut self, dst: usize) {}

    fn set_directions(&self, p: Passenger) -> Passenger {
        let dst = p.get_destination();
        p.set_next_direction(self.get_dir(dst))
            .set_next_stop(self.get_inter(dst))
    }

    fn get_dir(&self, dst: usize) -> usize {
        self.direction.get_direction(self.index, dst)
    }

    fn get_inter(&self, dst: usize) -> usize {
        self.interchange.next_station(self.index, dst)
    }
}

impl<'a> PassengerStation for Station<'a> {
    fn enter_passenger(&mut self, p: Passenger) {
        let p = self.set_directions(p);
        self.passengers.push(p)
    }
}

impl<'a> BoardPassengers for Station<'a> {
    fn board_passengers(&mut self, car: &mut Car) {
        let dst = car.get_destination();
        let passengers = self.passengers.get_list_mut(dst);
        car.board_passengers(passengers);
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
