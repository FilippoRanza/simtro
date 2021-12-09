use crate::line::LineDirection;
use crate::passenger::{Passenger, PassengerNextStopIndex};

use crate::line::SegmentType;
use crate::utils::counter::Counter;

use crate::utils::index_list::IndexList;

pub struct Car {
    passengers: IndexList<Passenger, PassengerNextStopIndex>,
    state: CarState,
    destination: usize,
    current: usize,
    direction: LineDirection,
    counter: Counter,
}

impl Car {
    pub fn new(
        destination: usize,
        current: usize,
        direction: LineDirection,
        line_size: usize,
    ) -> Self {
        Self {
            destination,
            current,
            direction,
            counter: Counter::new(0),
            state: CarState::Stop(0),
            passengers: IndexList::new_with_default_index(line_size),
        }
    }

    pub fn set_current(&mut self, stat: usize) {
        self.current = stat;
    }

    pub fn unboard_passengers(&mut self) -> &'_ mut Vec<Passenger> {
        self.passengers.get_list_mut(self.current)
    }

    pub fn board_passengers(&mut self, ps: &mut Vec<Passenger>) {
        self.passengers.append(ps);
    }

    pub fn board_passenger(&mut self, p: Passenger) {
        self.passengers.push(p);
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

    pub fn is_at_terminus(&self) -> bool {
        self.at_station(self.destination)
    }

    pub fn change_direction(&mut self) {
        self.direction.swap();
    }

    pub fn get_current_station(&self) -> usize {
        self.current
    }

    pub fn get_direction(&self) -> LineDirection {
        self.direction
    }

    pub fn run_step(&mut self) -> bool {
        self.counter.step()
    }

    pub fn next_step(&mut self, value: usize, kind: SegmentType) {
        self.counter = value.into();
        self.update_state(kind, 45);
    }

    fn update_state(&mut self, kind: SegmentType, next: usize) {
        self.state = match kind {
            SegmentType::Line => CarState::Rail,
            SegmentType::Station => CarState::Stop(next),
            SegmentType::Terminus => CarState::Stop(next),
        };
        if matches! {kind, SegmentType::Terminus} {
            self.direction.swap();
        }
    }
}

pub enum CarState {
    Stop(usize),
    Rail,
}
