//!
//!  This module contains the Passenger implementation. At the current time
//!  it is just an information about the start and stop station combined with a unique id.
//!  There is also the implementation for the passengere factory. This struct
//!  is initialized using a traffic matrix (see ``traffic_generator.rs``)
//!  iterates through this matrix and generate the required number of
//!  passengers that start from station i to stattion j
//!
use crate::station::PassengerStation;
use crate::traffic_generator::TrafficGenerator;
use crate::utils;
use crate::utils::index_list;
use rayon::prelude::*;

/// Passenger struct. Keep information about the
/// departure and destionaton station, with an unique id.
#[derive(Debug)]
pub struct Passenger {
    id: u32,
    start: usize,
    next_dir: usize,
    next_stop: usize,
    dest: usize,
}

impl Passenger {
    /// Create a new passenger instance.
    #[must_use]
    fn new(id: u32, start: usize, dest: usize) -> Self {
        Self {
            id,
            start,
            next_dir: 0,
            next_stop: 0,
            dest,
        }
    }

    /// Check if passenger is at its final destionation
    #[must_use]
    pub fn is_destination(&self, station: usize) -> bool {
        self.next_stop == station
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
}

impl utils::unique_id::SetId for Passenger {
    fn set_id(mut self, id: u32) -> Self {
        self.id = id;
        self
    }
}

#[derive(Default, Debug)]
pub struct PassengerNextStopIndex {}

impl index_list::Indexer<Passenger> for PassengerNextStopIndex {
    fn index(&self, p: &Passenger) -> usize {
        p.next_stop
    }
}

#[derive(Default)]
pub struct PassengerNextDirectionIndex {}

impl index_list::Indexer<Passenger> for PassengerNextDirectionIndex {
    fn index(&self, p: &Passenger) -> usize {
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
    pub fn generate_traffic<S: PassengerStation>(&self, step: u32, stations: &mut [S]) {
        self.traffic_generator
            .par_iter()
            .zip(stations.par_iter_mut())
            .enumerate()
            .for_each(|(i, (g, s))| Self::build_station_traffic(i, g, s, step));
    }

    /// Create passengers for the given station implementation.
    fn build_station_traffic<S: PassengerStation>(
        index: usize,
        traff_gen: &[Option<T>],
        stat: &mut S,
        step: u32,
    ) {
        for (dst, gen) in traff_gen.iter().enumerate() {
            if let Some(gen) = gen {
                for _ in 0..gen.next_traffic_flow(step) {
                    let p = Passenger::new(0, index, dst);
                    stat.enter_passenger(p);
                }
            }
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;

    impl PassengerStation for Vec<Passenger> {
        fn enter_passenger(&mut self, p: Passenger) {
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
        pass_factory.generate_traffic(0, &mut stations);
        assert_eq!(stations.len(), 3);
        assert_eq!(stations[0].len(), 9);
        assert_eq!(stations[1].len(), 5);
        assert_eq!(stations[2].len(), 3);
    }
}
