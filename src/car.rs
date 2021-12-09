use crate::line::LineDirection;
use crate::passenger::{Passenger, PassengerNextStopIndex};

use crate::line::SegmentType;
use crate::utils::counter::Counter;

use crate::utils::index_list::IndexList;

pub struct Car {
    passengers: IndexList<Passenger, PassengerNextStopIndex>,
    destination: usize,
    location: CarLocation,
    direction: LineDirection,
    counter: Counter,
}

impl Car {
    pub fn new(
        destination: usize,
        location: CarLocation,
        direction: LineDirection,
        line_size: usize,
    ) -> Self {
        Self {
            destination,
            location,
            direction,
            counter: Counter::new(0),
            passengers: IndexList::new_with_default_index(line_size),
        }
    }

    pub fn set_location(&mut self, loc: CarLocation) {
        self.location = loc
    }

    pub fn unboard_passengers(&mut self) -> &'_ mut Vec<Passenger> {
        self.passengers.get_list_mut(self.get_current_station())
    }

    pub fn board_passengers(&mut self, ps: &mut Vec<Passenger>) {
        self.passengers.append(ps);
    }

    pub fn in_station(&self) -> bool {
        matches! {self.location, CarLocation::Station{ station: _, segment: _ }}
    }

    pub fn at_station(&self, s: usize) -> bool {
        matches! {self.location, CarLocation::Station{ station, segment: _ } if s == station}
    }

    pub fn get_destination(&self) -> usize {
        self.destination
    }

    pub fn change_direction(&mut self) {
        self.direction.swap();
    }

    pub fn get_current_station(&self) -> usize {
        self.location.get_station()
    }

    pub fn get_current_segment(&self) -> usize {
        self.location.get_segment()
    }

    pub fn get_direction(&self) -> LineDirection {
        self.direction
    }

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

pub enum CarLocation {
    Segment { index: usize },
    Station { segment: usize, station: usize },
}

impl CarLocation {
    pub fn segment(index: usize) -> Self {
        Self::Segment { index }
    }

    pub fn station(segment: usize, station: usize) -> Self {
        Self::Station { segment, station }
    }

    pub fn get_station(&self) -> usize {
        match self {
            Self::Station {
                segment: _,
                station,
            } => *station,
            _ => panic! {},
        }
    }

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
