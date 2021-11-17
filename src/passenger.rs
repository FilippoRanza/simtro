/*
   This file contains the Passenger implementation. At the current time
   it is just an information about the start and stop station combined with a unique id.

   There is also the implementation for the passengere factory. This struct
   is initialized using a traffic matrix (see traffic_generator.rs)
   iterates through this matrix and generate the required number of
   passengers that start from station i to stattion j
*/

use crate::utils;
use ndarray;

pub struct Passenger {
    id: u32,
    start: usize,
    stop: usize,
}

impl Passenger {
    fn new(id: u32, start: usize, stop: usize) -> Self {
        Self { id, start, stop }
    }

    pub fn is_destination(&self, station: usize) -> bool {
        self.stop == station
    }
}

impl utils::unique_id::SetId for Passenger {
    fn set_id(mut self, id: u32) -> Self {
        self.id = id;
        self
    }
}

pub struct PassengerFactory<'a> {
    lambda_matrix: &'a ndarray::Array2<u32>,
    row: usize,
}

impl<'a> PassengerFactory<'a> {
    pub fn new(lambda_matrix: &'a ndarray::Array2<u32>) -> Self {
        Self {
            lambda_matrix,
            row: 0,
        }
    }
}

impl<'a> Iterator for PassengerFactory<'a> {
    type Item = PassengerGeneratorIterator<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.row < self.lambda_matrix.nrows() {
            let row_iter = self.lambda_matrix.row(self.row).into_iter();
            let out = Some(PassengerGeneratorIterator::new(row_iter, self.row));
            self.row += 1;
            out
        } else {
            None
        }
    }
}

type Row<'a> = ndarray::iter::Iter<'a, u32, ndarray::Dim<[usize; 1]>>;

pub struct PassengerGeneratorIterator<'a> {
    row: Row<'a>,
    missing: u32,
    departure: usize,
    destination: usize,
}

impl<'a> PassengerGeneratorIterator<'a> {
    fn new(row: Row<'a>, departure: usize) -> Self {
        Self {
            row,
            departure,
            missing: 0,
            destination: 0,
        }
    }

    fn get_station(&mut self) -> Option<usize> {
        while self.missing == 0 {
            self.missing = *self.row.next()?;
            self.destination += 1;
        }
        self.missing -= 1;
        Some(self.destination)
    }
}

impl<'a> Iterator for PassengerGeneratorIterator<'a> {
    type Item = Passenger;
    fn next(&mut self) -> Option<Self::Item> {
        let destination = self.get_station()?;
        Some(Passenger::new(0, self.departure, destination))
    }
}

#[cfg(test)]
mod test {

    use super::*;

    use ndarray::arr2;

    #[test]
    fn test_passenger_count() {
        /*
            Ensure that the number of passengers generated
            is correct. Check correct behavior with zero passengers
            to create.
        */
        let lambda_matrix = arr2(&[[0, 4, 1], [4, 0, 2], [5, 1, 0]]);
        let mut uid_gen = utils::unique_id::UniqueId::new();
        let factory = PassengerFactory::new(&lambda_matrix);
        let mut passengers = Vec::new();
        for iter in factory {
            for p in uid_gen.set_id_iter(iter) {
                passengers.push(p)
            }
        }
        assert_eq!(passengers.len(), 0 + 4 + 1 + 4 + 0 + 2 + 5 + 1 + 0);
        for (i, p) in passengers.iter().enumerate() {
            assert_eq!(p.id, i as u32);
        }
    }
}
