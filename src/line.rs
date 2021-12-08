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

use crate::station::BoardPassengers;

/// Control the current state of
/// a given metro line.
pub struct Line {
    train_counter: counter::Counter,
    terminus_a: Terminus,
    terminus_b: Terminus,
    dir: Railway,
    fleet: fleet::Fleet,
    line_size: usize,
}

/// Allow to specify if
/// direction is from terminus 1 to terminus 2
/// or vice versa
#[derive(Clone, Copy)]
pub enum LineDirection {
    DirectionA,
    DirectionB,
}

impl LineDirection {
    /// invert direction
    pub fn swap(&mut self) {
        *self = match self {
            Self::DirectionA => Self::DirectionB,
            Self::DirectionB => Self::DirectionA,
        };
    }

    /// Choose first element if DirectionA, choose second if DirectionB
    fn choose_direction<T>(&self, t1: T, t2: T) -> T {
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
        dir: Railway,
        fleet: fleet::Fleet,
        line_size: usize
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
            line_size
        }
    }

    /// Implement a simulation step. Move trains
    /// and, if it is possible, start a new train
    pub fn step(&mut self) {
        self.move_train();
        self.start_train();
        self.terminus_a.step();
        self.terminus_b.step();
    }

    /// Board passengers on the train from the current
    /// station.
    pub fn passenger_boarding(&mut self, stats: &mut [station::Station]) {
        for car in self.fleet.in_station_car_iter() {
            let station = &mut stats[car.get_current_station()];
            station.board_passengers(car)
        }
    }

    /// Step each train on the line.
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

    /// Start train on each direction
    fn start_train(&mut self) {
        self.try_start_new_train(LineDirection::DirectionA);
        self.try_start_new_train(LineDirection::DirectionB);
    }

    /// Try to start a train in a given direction
    fn try_start_new_train(&mut self, dir: LineDirection) {
        if self.can_start_new_train(dir) {
            self.start_new_train(dir);
        }
    }

    /// Check if it is possible to start a new train
    fn can_start_new_train(&self, dir: LineDirection) -> bool {
        if !self.train_counter.is_done() {
            return false;
        }
        if !self.dir.get_terminus(dir).is_free(dir) {
            return false;
        }
        self.get_terminus(dir).can_start_new_train()
    }

    /// actually start a train
    fn start_new_train(&mut self, dir: LineDirection) {
        self.get_terminus_mut(dir).add_new_train();
        self.train_counter.step();
        let s_id = self.get_terminus(dir).get_station_id();
        let car = car::Car::new(s_id, s_id, dir, self.line_size);
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

/// Implement the railway line. A Railway line is made of
/// trunks.
pub struct Railway {
    line: Vec<Segment>,
}

impl Railway {
    fn new(line: Vec<Segment>) -> Self {
        Railway { line }
    }

    /// Check if it is possible to occupy the next trunk (relative to direction)
    /// If it is possible perform the actual truck state update and return info
    /// about the next step
    fn next_step(&mut self, curr: usize, dir: LineDirection) -> Option<NextStepInfo> {
        if self.is_free(curr, dir) {
            Some(self.update_car_location(curr, dir))
        } else {
            None
        }
    }

    /// Check if the next step is free or not.
    fn is_free(&self, curr: usize, dir: LineDirection) -> bool {
        let next = get_next_trunk(curr, dir);
        self.line[next].is_free(dir)
    }

    /// update truck state: free previous and occupy current.
    fn update_car_location(&mut self, curr: usize, dir: LineDirection) -> NextStepInfo {
        let next = get_next_trunk(curr, dir);
        self.line[next].set_occupied(dir);
        self.line[curr].set_free(dir);
        NextStepInfo {
            kind: self.line[next].get_type(dir),
            time: self.line[next].get_duration(dir),
        }
    }

    /// Get terminus for given direction
    fn get_terminus(&self, dir: LineDirection) -> &'_ Segment {
        dir.choose_direction(self.line.first().unwrap(), self.line.last().unwrap())
    }
}

struct NextStepInfo {
    kind: SegmentType,
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
    station_id: usize,
    depo_counter: counter::Counter,
    train_counter: counter::CyclicCounter,
}

impl Terminus {
    fn new<D, T>(id: usize, depo_size: D, train_delay: T) -> Self
    where
        D: Into<counter::Counter>,
        T: Into<counter::CyclicCounter>,
    {
        Self {
            station_id: id,
            depo_counter: depo_size.into(),
            train_counter: train_delay.into(),
        }
    }

    fn can_start_new_train(&self) -> bool {
        if self.depo_counter.is_done() {
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
    }

    fn get_station_id(&self) -> usize {
        self.station_id
    }
}

/// A railway segemnt. It can be single, so it allows just
/// one car at the time or double so it is possible to have one car
/// for direction.
enum Segment {
    Single(SegmentInfo),
    Double(SegmentInfo, SegmentInfo),
}

impl Segment {
    fn is_free(&self, dir: LineDirection) -> bool {
        self.choose_segment_info(dir).is_free()
    }

    fn set_occupied(&mut self, dir: LineDirection) {
        self.choose_segment_info_mut(dir).set_occupied();
    }

    fn set_free(&mut self, dir: LineDirection) {
        self.choose_segment_info_mut(dir).set_free();
    }

    fn get_duration(&self, dir: LineDirection) -> usize {
        self.choose_segment_info(dir).get_duration()
    }

    fn get_type(&self, dir: LineDirection) -> SegmentType {
        self.choose_segment_info(dir).get_type()
    }

    fn choose_segment_info_mut(&mut self, dir: LineDirection) -> &'_ mut SegmentInfo {
        match self {
            Self::Single(ti) => ti,
            Self::Double(d1, d2) => dir.choose_direction(d1, d2),
        }
    }

    fn choose_segment_info(&self, dir: LineDirection) -> &'_ SegmentInfo {
        match self {
            Self::Single(ti) => ti,
            Self::Double(d1, d2) => dir.choose_direction(d1, d2),
        }
    }
}

/// Information about a network segment
/// type of segment, status and duration to
/// to traverse it
struct SegmentInfo {
    kind: SegmentType,
    stat: SegmentStatus,
    duration: usize,
}

impl SegmentInfo {
    fn is_free(&self) -> bool {
        matches! {self.stat, SegmentStatus::Free}
    }

    fn set_occupied(&mut self) {
        self.stat = SegmentStatus::Occupied
    }

    fn set_free(&mut self) {
        self.stat = SegmentStatus::Free
    }

    fn get_duration(&self) -> usize {
        self.duration
    }

    fn get_type(&self) -> SegmentType {
        self.kind
    }
}

/// A railway segment can be a station,
/// a line connecting stations or a terminus (station
/// where a train must change direction)
#[derive(Clone, Copy)]
pub enum SegmentType {
    Station,
    Terminus,
    Line,
}

/// A segment can be free or occupied
enum SegmentStatus {
    Free,
    Occupied,
}

impl Default for SegmentStatus {
    fn default() -> Self {
        Self::Free
    }
}
