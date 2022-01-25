//! This module contains some trait defining a general
//! behavior that a station must implement and offers also
//! a simple implementation of those traits.

use crate::car::Car;
use crate::passenger::{callbacks, Passenger, PassengerNextDirectionIndex};
use crate::routes::{MetroDirection, MetroInterchange};
use crate::utils::index_list;

pub trait StationClass {
    fn is_terminus(&self) -> bool;
    fn is_interchange(&self) -> bool;
}

/// Define general purpose method for a passenger
/// station. Trait main usage is to improve
/// client code generality.
pub trait PassengerStation<T>: Send + Sync {
    /// Add passengers to the station implementation
    fn enter_passenger(&mut self, p: Passenger<T>);
}

/// Board passengers on given train
pub trait BoardPassengers<T>: Send + Sync {
    /// Board passenger on given ``Car``. Boarded passengers
    /// will no longer be on object
    fn board_passengers(&mut self, c: &mut Car<T>);
}

/// Land passengers from train to current implementation
pub trait LandPassenger<T>: Send + Sync {
    /// Land passenger from given Car in trait implementation.
    /// Landed train will no longer be inside given car
    fn land_passenger(&mut self, c: &mut Car<T>);
}

#[must_use]
pub fn station_list_factory<'a, T>(
    count: usize,
    direction: &'a MetroDirection,
    interchange: &'a MetroInterchange,
) -> Vec<Station<'a, T>> {
    (0..count)
        .map(|id| station_factory(id, count, direction, interchange))
        .collect()
}

fn station_factory<'a, T>(
    id: usize,
    total: usize,
    direction: &'a MetroDirection,
    interchange: &'a MetroInterchange,
) -> Station<'a, T> {
    let passengers = index_list::IndexList::new_with_default_index(total);
    Station {
        index: id,
        direction,
        interchange,
        passengers,
    }
}

/// Simple station implementation.
/// Contains information about the
/// station id, ``MetroDirection`` and ``MetroIntechage`` and
/// the passenger list
pub struct Station<'a, T> {
    index: usize,
    direction: &'a MetroDirection,
    interchange: &'a MetroInterchange,
    passengers: index_list::IndexList<Passenger<T>, PassengerNextDirectionIndex>,
}

impl<'a, T> Station<'a, T>
where
    T: callbacks::PassengerAction,
{
    fn set_directions(&self, p: Passenger<T>) -> Passenger<T> {
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

impl<'a, T> PassengerStation<T> for Station<'a, T>
where
    T: callbacks::PassengerAction,
{
    fn enter_passenger(&mut self, p: Passenger<T>) {
        let p = self.set_directions(p).enter_station();
        self.passengers.push(p);
    }
}

impl<'a, T> BoardPassengers<T> for Station<'a, T>
where
    T: callbacks::PassengerAction,
{
    fn board_passengers(&mut self, car: &mut Car<T>) {
        let dst = car.get_destination();
        let passengers = self.passengers.get_list_mut(dst);
        car.board_passengers(passengers);
    }
}

impl<'a, T> LandPassenger<T> for Station<'a, T>
where
    T: callbacks::PassengerAction,
{
    fn land_passenger(&mut self, c: &mut Car<T>) {
        let passenger = c.unboard_passengers();
        let iter = passenger
            .drain(..)
            .map(Passenger::leave_train)
            .filter(|p| p.is_destination(self.index));
        self.passengers.append_iter(iter);
    }
}
