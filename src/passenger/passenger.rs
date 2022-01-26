//!
//!  This module contains the Passenger implementation. At the current time
//!  it is just an information about the start and stop station combined with a unique id.
//!  There is also the implementation for the passengere factory. This struct
//!  is initialized using a traffic matrix (see ``traffic_generator.rs``)
//!  iterates through this matrix and generate the required number of
//!  passengers that start from station i to stattion j
//!
use super::callbacks::{PassengerAction, PassengerActionFactory};
use crate::station::PassengerStation;
use crate::traffic_generator::TrafficGenerator;
use crate::utils;
use crate::utils::index_list;

/// Passenger struct. Keep information about the
/// departure and destionaton station, with an unique id.
#[derive(Debug)]
pub struct Passenger<T> {
    id: u32,
    start: usize,
    next_dir: usize,
    next_stop: usize,
    dest: usize,
    callback: T,
}

impl<T> Passenger<T>
where
    T: PassengerAction,
{
    /// Create a new passenger instance.
    #[must_use]
    fn new(id: u32, start: usize, dest: usize, callback: T) -> Self {
        Self {
            id,
            start,
            next_dir: 0,
            next_stop: 0,
            dest,
            callback,
        }
    }

    /// Check if passenger is at its intermediate destionation
    #[must_use]
    pub fn is_destination(&self, station: usize) -> bool {
        self.next_stop == station
    }

    #[must_use]
    pub fn is_final_destination(&self, station: usize) -> bool {
        self.dest == station
    }


    /// Set next direction - terminus station - to reach
    /// destination
    #[must_use]
    pub fn set_next_direction(mut self, dir: usize) -> Self {
        self.next_dir = dir;
        self
    }

    /// Set next interchange - terminus station - to reach
    /// destination
    #[must_use]
    pub fn set_next_stop(mut self, stop: usize) -> Self {
        self.next_stop = stop;
        self
    }

    /// return passenger destination
    #[must_use]
    pub fn get_destination(&self) -> usize {
        self.dest
    }

    #[must_use]
    pub fn enter_station(mut self) -> Self {
        self.callback.enter_station(self.start);
        self
    }

    #[must_use]
    pub fn leave_train(mut self) -> Self {
        self.callback.leave_train(self.next_stop);
        self
    }
}

impl<T> utils::unique_id::SetId for Passenger<T> {
    fn set_id(mut self, id: u32) -> Self {
        self.id = id;
        self
    }
}

#[derive(Default, Debug)]
pub struct PassengerNextStopIndex {}

impl<T> index_list::Indexer<Passenger<T>> for PassengerNextStopIndex {
    fn index(&self, p: &Passenger<T>) -> usize {
        p.next_stop
    }
}

#[derive(Default)]
pub struct PassengerNextDirectionIndex {}

impl<T> index_list::Indexer<Passenger<T>> for PassengerNextDirectionIndex {
    fn index(&self, p: &Passenger<T>) -> usize {
        p.next_dir
    }
}

/// Create new passengers for each station going to each
/// station. At each simulation step create passengers
/// according to the number given by the traffic generator
/// implementation.
pub struct PassengerFactory<T> {
    traffic_generator: Vec<Vec<Option<T>>>,
}

impl<T> PassengerFactory<T>
where
    T: TrafficGenerator,
{
    /// Initialize factory. T initialization is now handled here.
    #[must_use]
    pub fn new(traffic_generator: Vec<Vec<Option<T>>>) -> Self {
        Self { traffic_generator }
    }

    /// Generate traffic at given step. Borrows mutable the list of all stations.
    pub fn generate_traffic<Pc: PassengerAction, S: PassengerStation<Pc>, Tf>(
        &self,
        step: u32,
        stations: &mut [S],
        tf: &mut Tf,
    ) where
        Tf: PassengerActionFactory<Pc> + Send + Sync,
    {
        self.traffic_generator
            .iter()
            .zip(stations.iter_mut())
            .enumerate()
            .for_each(|(i, (g, s))| Self::build_station_traffic(i, g, s, step, tf));
    }

    /// Create passengers for the given station implementation.
    fn build_station_traffic<Pc: PassengerAction, S: PassengerStation<Pc>, Tf>(
        index: usize,
        traff_gen: &[Option<T>],
        stat: &mut S,
        step: u32,
        tf: &mut Tf,
    ) where
        Tf: PassengerActionFactory<Pc>,
    {
        for (dst, gen) in traff_gen.iter().enumerate() {
            if let Some(gen) = gen {
                for _ in 0..gen.next_traffic_flow(step) {
                    let p = Passenger::new(0, index, dst, tf.factory());
                    stat.enter_passenger(p);
                }
            }
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;

    impl<T> PassengerStation<T> for Vec<Passenger<T>>
    where
        T: PassengerAction,
    {
        fn enter_passenger(&mut self, p: Passenger<T>) {
            self.push(p);
        }
    }

    impl TrafficGenerator for u32 {
        fn next_traffic_flow(&self, _step: u32) -> u32 {
            *self
        }
    }

    #[test]
    fn test_passenger_count() {
        /*
            Ensure that the number of passengers generated
            is correct. Check correct behavior with zero passengers
            to create.
        */
        let traffic_generator = vec![
            vec![None, Some(4), Some(5)],
            vec![Some(3), None, Some(2)],
            vec![Some(1), Some(2), None],
        ];

        let mut stations = vec![vec![], vec![], vec![]];

        let pass_factory = PassengerFactory::new(traffic_generator);
        pass_factory.generate_traffic(0, &mut stations, &mut ());
        assert_eq!(stations.len(), 3);
        assert_eq!(stations[0].len(), 9);
        assert_eq!(stations[1].len(), 5);
        assert_eq!(stations[2].len(), 3);
    }
}
