//! This module control train line.
//! A train line - for this module business - is made of
//! two terminus, information about the number
//! of trains running, information about the
//! maximal number of trains allowd on the line
//! and current occupation of stations and connecting
//! railways.

use crate::car;
use crate::fleet;
use crate::station;
use crate::utils::counter;

/// Control the current state of
/// a given metro line.
pub struct Line {
    train_counter: counter::Counter,
    terminus_a: Terminus,
    terminus_b: Terminus,
    dir: Direction,
    fleet: fleet::Fleet,
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
            Self::DirectionB => Self::DirectionA,
        };
    }

    fn choose_direction<'a, T>(&self, t1: T, t2: T) -> T {
        match self {
            Self::DirectionA => t1,
            Self::DirectionB => t2,
        }
    }
}

impl Line {
    fn new<C>(
        counter: C,
        terminus_a: Terminus,
        terminus_b: Terminus,
        dir: Direction,
        fleet: fleet::Fleet,
    ) -> Self
    where
        C: Into<counter::Counter>,
    {
        Line {
            train_counter: counter.into(),
            terminus_a,
            terminus_b,
            dir,
            fleet,
        }
    }

    pub fn step(&mut self) {
        self.move_train();
        self.start_train();
    }

    pub fn passenger_boarding(&mut self, stats: &mut [station::Station]) {
        for car in self.fleet.in_station_car_iter() {
            let station = &stats[car.get_current_station()];
            todo! {}
        }
    }

    fn move_train(&mut self) {
        for train in self.fleet.running_cars_iter() {
            if train.run_step() {
                if let Some(NextStepInfo { time, kind }) = self
                    .dir
                    .next_step(train.get_current_station(), train.get_direction())
                {
                    train.next_step(time, kind);
                }
            }
        }
    }

    fn start_train(&mut self) {
        self.try_start_new_train(LineDirection::DirectionA);
        self.try_start_new_train(LineDirection::DirectionB);
    }

    fn try_start_new_train(&mut self, dir: LineDirection) {
        if self.can_start_new_train(dir) {
            self.start_new_train(dir);
        }
    }

    fn can_start_new_train(&self, dir: LineDirection) -> bool {
        if !self.train_counter.is_done() {
            return false;
        }
        if !self.dir.get_terminus(dir).is_free(dir) {
            return false;
        }
        self.get_terminus(dir).can_start_new_train()
    }

    fn start_new_train(&mut self, dir: LineDirection) {
        self.get_terminus_mut(dir).add_new_train();
        self.train_counter.step();
        let car = car::Car::new(0, 1, dir);
        self.fleet.start_train(car);
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

pub struct Direction {
    line: Vec<Trunk>,
}

impl Direction {
    fn new(line: Vec<Trunk>) -> Self {
        Self { line }
    }

    fn next_step(&mut self, curr: usize, dir: LineDirection) -> Option<NextStepInfo> {
        if self.is_free(curr, dir) {
            Some(self.update_car_location(curr, dir))
        } else {
            None
        }
    }

    fn is_free(&self, curr: usize, dir: LineDirection) -> bool {
        let next = get_next_trunk(curr, dir);
        self.line[next].is_free(dir)
    }

    fn update_car_location(&mut self, curr: usize, dir: LineDirection) -> NextStepInfo {
        let next = get_next_trunk(curr, dir);
        self.line[next].set_occupied(dir);
        self.line[curr].set_free(dir);
        NextStepInfo {
            kind: self.line[next].get_type(dir),
            time: self.line[next].get_duration(dir),
        }
    }

    fn get_terminus(&self, dir: LineDirection) -> &'_ Trunk {
        dir.choose_direction(self.line.first().unwrap(), self.line.last().unwrap())
    }
}

struct NextStepInfo {
    kind: TrunkType,
    time: usize,
}

fn get_next_trunk(curr: usize, dir: LineDirection) -> usize {
    dir.choose_direction(curr + 1, curr - 1)
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

enum Trunk {
    Single(TrunkInfo),
    Double(TrunkInfo, TrunkInfo),
}

impl Trunk {
    fn is_free(&self, dir: LineDirection) -> bool {
        self.choose_trunk_info(dir).is_free()
    }

    fn set_occupied(&mut self, dir: LineDirection) {
        self.choose_trunk_info_mut(dir).set_occupied();
    }

    fn set_free(&mut self, dir: LineDirection) {
        self.choose_trunk_info_mut(dir).set_free();
    }

    fn get_duration(&self, dir: LineDirection) -> usize {
        self.choose_trunk_info(dir).get_duration()
    }

    fn get_type(&self, dir: LineDirection) -> TrunkType {
        self.choose_trunk_info(dir).get_type()
    }

    fn choose_trunk_info_mut(&mut self, dir: LineDirection) -> &'_ mut TrunkInfo {
        match self {
            Self::Single(ti) => ti,
            Self::Double(d1, d2) => dir.choose_direction(d1, d2),
        }
    }

    fn choose_trunk_info(&self, dir: LineDirection) -> &'_ TrunkInfo {
        match self {
            Self::Single(ti) => ti,
            Self::Double(d1, d2) => dir.choose_direction(d1, d2),
        }
    }
}

struct TrunkInfo {
    kind: TrunkType,
    stat: TrunkStatus,
    duration: usize,
}

impl TrunkInfo {
    fn is_free(&self) -> bool {
        matches! {self.stat, TrunkStatus::Free}
    }

    fn set_occupied(&mut self) {
        self.stat = TrunkStatus::Occupied
    }

    fn set_free(&mut self) {
        self.stat = TrunkStatus::Free
    }

    fn get_duration(&self) -> usize {
        self.duration
    }

    fn get_type(&self) -> TrunkType {
        self.kind
    }
}

#[derive(Clone, Copy)]
pub enum TrunkType {
    Station,
    Terminus,
    Line,
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
