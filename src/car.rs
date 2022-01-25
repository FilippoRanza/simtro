use crate::line::LineDirection;
use crate::passenger::{Passenger, PassengerNextStopIndex};

use crate::line::SegmentType;
use crate::utils::counter::Counter;

use crate::utils::index_list::IndexList;

#[derive(Debug)]
pub struct Car<T> {
    passengers: IndexList<Passenger<T>, PassengerNextStopIndex>,
    destination: usize,
    location: CarLocation,
    direction: LineDirection,
    counter: Counter,
}

impl<T> Car<T> {
    #[must_use]
    pub fn new(
        destination: usize,
        location: CarLocation,
        direction: LineDirection,
        network_size: usize,
        station_len: usize,
    ) -> Self {
        Self {
            destination,
            location,
            direction,
            counter: station_len.into(),
            passengers: IndexList::new_with_default_index(network_size),
        }
    }

    pub fn set_location(&mut self, loc: CarLocation) {
        self.location = loc;
    }

    pub fn unboard_passengers(&mut self) -> &'_ mut Vec<Passenger<T>> {
        self.passengers.get_list_mut(self.get_current_station())
    }

    pub fn board_passengers(&mut self, ps: &mut Vec<Passenger<T>>) {
        self.passengers.append(ps);
    }

    #[must_use]
    pub fn in_station(&self) -> bool {
        matches! {self.location, CarLocation::Station{ station: _, segment: _ }}
    }

    #[must_use]
    pub fn at_station(&self, s: usize) -> bool {
        matches! {self.location, CarLocation::Station{ station, segment: _ } if s == station}
    }

    #[must_use]
    pub fn get_destination(&self) -> usize {
        self.destination
    }

    pub fn change_direction(&mut self) {
        self.direction.swap();
    }

    #[must_use]
    pub fn get_current_station(&self) -> usize {
        self.location.get_station()
    }

    #[must_use]
    pub fn get_current_segment(&self) -> usize {
        self.location.get_segment()
    }

    #[must_use]
    pub fn get_direction(&self) -> LineDirection {
        self.direction
    }

    #[must_use]
    pub fn run_step(&mut self) -> bool {
        self.counter.step()
    }

    pub fn next_step(&mut self, value: usize, kind: SegmentType, loc: CarLocation) {
        self.counter = value.into();
        self.location = loc;
        self.update_state(kind);
    }

    fn update_state(&mut self, kind: SegmentType) {
        if matches! {kind, SegmentType::Terminus(_)} {
            self.direction.swap();
        }
    }
}
#[derive(Debug)]
pub enum CarLocation {
    Segment { index: usize },
    Station { segment: usize, station: usize },
}

impl CarLocation {
    #[must_use]
    pub fn segment(index: usize) -> Self {
        Self::Segment { index }
    }

    #[must_use]
    pub fn station(segment: usize, station: usize) -> Self {
        Self::Station { segment, station }
    }

    #[must_use]
    pub fn get_station(&self) -> usize {
        match self {
            Self::Station {
                segment: _,
                station,
            } => *station,
            Self::Segment { .. } => panic! {},
        }
    }

    #[must_use]
    pub fn get_segment(&self) -> usize {
        match self {
            Self::Station {
                segment,
                station: _,
            } => *segment,
            Self::Segment { index } => *index,
        }
    }
}
