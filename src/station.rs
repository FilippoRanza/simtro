//! This module contains some trait defining a general
//! behavior that a station must implement and offers also
//! a simple implementation of those traits.

use crate::car::Car;
use crate::passenger::{Passenger, PassengerNextDirectionIndex};
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

/// Board passengers on given train
pub trait BoardPassengers: Send + Sync {
    /// Board passenger on given Car. Boarded passengers
    /// will no longer be on object
    fn board_passengers(&mut self, c: &mut Car);
}

/// Land passengers from train to current implementation
pub trait LandPassenger: Send + Sync {
    /// Land passenger from given Car in trait implementation.
    /// Landed train will no longer be inside given car
    fn land_passenger(&mut self, c: &mut Car);
}

/// Simple station implementation.
/// Contains information about the
/// station id, MetroDirection and MetroIntechage and
/// the passenger list
pub struct Station<'a> {
    index: usize,
    direction: &'a MetroDirection,
    interchange: &'a MetroInterchange,
    passengers: index_list::IndexList<Passenger, PassengerNextDirectionIndex>,
}

impl<'a> Station<'a> {
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

impl<'a> LandPassenger for Station<'a> {
    fn land_passenger(&mut self, c: &mut Car) {
        let passenger = c.unboard_passengers();
        let iter = passenger.drain(..).filter(|p| p.is_destination(self.index));
        self.passengers.append_iter(iter);
    }
}
