//! This module control train line.
//! A train line - for this module business - is made of
//! two terminus, information about the number
//! of trains running, information about the
//! maximal number of trains allowd on the line
//! and current occupation of stations and connecting
//! railways.

use crate::utils;
use crate::utils::counter;

/// Control the current state of
/// a given metro line.
pub struct Lines {
    train_counter: counter::Counter,
    terminus_a: Terminus,
    terminus_b: Terminus,
}

#[derive(Clone, Copy)]
pub enum LineDirection {
    DirectionA,
    DirectionB,
}

impl LineDirection {
    pub fn swap(&mut self) {
        *self = match self {
            Self::DirectionA => Self::DirectionB,
            Self::DirectionB => Self::DirectionA
        };
    }
}


impl Lines {
    pub fn new<C>(counter: C, terminus_a: Terminus, terminus_b: Terminus) -> Self
    where
        C: Into<counter::Counter>,
    {
        Self {
            train_counter: counter.into(),
            terminus_a,
            terminus_b,
        }
    }

    pub fn can_start_new_train(&self, dir: LineDirection) -> bool {
        if self.train_counter.is_done() {
            self.get_terminus(dir).can_start_new_train()
        } else {
            false
        }
    }

    pub fn start_new_train(&mut self, dir: LineDirection) {
        self.get_terminus_mut(dir).add_new_train();
        self.train_counter.step();
    }

    pub fn train_departure(&mut self, dir: LineDirection) {
        self.get_terminus_mut(dir).train_departure();
    }

    pub fn trian_arrival(&mut self, dir: LineDirection) {
        self.get_terminus_mut(dir).train_arrival();   
    }

    fn get_terminus(&self, dir: LineDirection) -> &'_ Terminus {
        match dir {
            LineDirection::DirectionA => &self.terminus_a,
            LineDirection::DirectionB => &self.terminus_b,
        }
    }

    fn get_terminus_mut(&mut self, dir: LineDirection) -> &'_ mut Terminus {
        match dir {
            LineDirection::DirectionA => &mut self.terminus_a,
            LineDirection::DirectionB => &mut self.terminus_b,
        }
    }
}

/// A terminus station can used
/// also a deposit. This struct
/// contains information about
/// the current number of trains
/// in the deposit and maximal number
/// of station in deposit.

pub struct Terminus {
    depo_counter: counter::Counter,
    train_counter: counter::CyclicCounter,
    state: TrunkStatus,
}

impl Terminus {
    fn new<D, T>(depo_size: D, train_delay: T) -> Self
    where
        D: Into<counter::Counter>,
        T: Into<counter::CyclicCounter>,
    {
        Self {
            depo_counter: depo_size.into(),
            train_counter: train_delay.into(),
            state: TrunkStatus::default(),
        }
    }

    fn can_start_new_train(&self) -> bool {
        if matches! {self.state, TrunkStatus::Occupied} {
            false
        } else if self.depo_counter.is_done() {
            false
        } else {
            !self.train_counter.is_done()
        }
    }

    fn step(&mut self) {
        self.train_counter.count();
    }

    fn add_new_train(&mut self) {
        self.depo_counter.step();
        self.train_arrival();
    }

    fn train_arrival(&mut self) {
        self.state = TrunkStatus::Occupied;
    }

    fn train_departure(&mut self) {
        self.state = TrunkStatus::Free;
    }
}

enum TrunkStatus {
    Free,
    Occupied,
}
impl Default for TrunkStatus {
    fn default() -> Self {
        Self::Free
    }
}

fn init_status(stations: usize) -> (StationState, RailwayState) {
    (StationState::new(stations), RailwayState::new(stations - 1))
}

struct StationState {
    state: Vec<TrunkStatus>,
}

impl StationState {
    fn new(sz: usize) -> Self {
        Self {
            state: utils::zeros(sz),
        }
    }
}

struct RailwayState {
    state: Vec<TrunkStatus>,
}

impl RailwayState {
    fn new(sz: usize) -> Self {
        Self {
            state: utils::zeros(sz),
        }
    }
}
