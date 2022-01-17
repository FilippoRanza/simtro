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

use crate::station::{BoardPassengers, LandPassenger};

/// Control the current state of
/// a given metro line.
pub struct Line {
    train_counter: counter::Counter,
    terminus_a: Terminus,
    terminus_b: Terminus,
    railway: Railway,
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
        railway: Railway,
        fleet: fleet::Fleet,
        line_size: usize,
    ) -> Self
    where
        C: Into<counter::Counter>,
    {
        Line {
            train_counter: counter.into(),
            terminus_a,
            terminus_b,
            railway,
            fleet,
            line_size,
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
    pub fn boarding_operations(&mut self, stats: &mut [station::Station]) {
        for car in self.fleet.in_station_car_iter() {
            let station = &mut stats[car.get_current_station()];
            station.board_passengers(car);
            station.land_passenger(car);
        }
    }

    /// Step each train on the line.
    fn move_train(&mut self) {
        for train in self.fleet.running_cars_iter() {
            if train.run_step() {
                if let Some(NextStepInfo { time, kind, loc }) = self
                    .railway
                    .next_step(train.get_current_segment(), train.get_direction())
                {
                    train.next_step(time, kind, loc);
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
        if !self.railway.get_terminus(dir).is_free(dir) {
            return false;
        }
        self.get_terminus(dir).can_start_new_train()
    }

    /// actually start a train
    fn start_new_train(&mut self, dir: LineDirection) {
        self.get_terminus_mut(dir).add_new_train();
        self.train_counter.step();
        let station_index = self.get_terminus(dir).get_station_id();
        let segment_index = self.get_terminus_index(dir);
        let location = car::CarLocation::station(segment_index, station_index);
        let car = car::Car::new(station_index, location, dir, self.line_size);
        self.fleet.start_train(car);
    }

    fn get_terminus_index(&self, dir: LineDirection) -> usize {
        dir.choose_direction(self.railway.last_index(), 0)
    }

    fn get_terminus(&self, dir: LineDirection) -> &'_ Terminus {
        dir.choose_direction(&self.terminus_a, &self.terminus_b)
    }

    fn get_terminus_mut(&mut self, dir: LineDirection) -> &'_ mut Terminus {
        dir.choose_direction(&mut self.terminus_a, &mut self.terminus_b)
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
            loc: self.line[next].make_location(dir, next),
        }
    }

    /// Get terminus for given direction
    fn get_terminus(&self, dir: LineDirection) -> &'_ Segment {
        dir.choose_direction(self.line.first().unwrap(), self.line.last().unwrap())
    }

    fn last_index(&self) -> usize {
        self.line.len() - 1
    }
}

struct NextStepInfo {
    kind: SegmentType,
    time: usize,
    loc: car::CarLocation,
}

fn get_next_trunk(curr: usize, dir: LineDirection) -> usize {
    match dir {
        LineDirection::DirectionA => curr - 1,
        LineDirection::DirectionB => curr + 1,
    }
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
            self.train_counter.is_done()
        }
    }

    fn step(&mut self) {
        self.train_counter.count();
    }

    fn add_new_train(&mut self) {
        self.depo_counter.step();
    }

    /// Provide global station index of this terminus.
    /// Required by the train to specify its direction.
    fn get_station_id(&self) -> usize {
        self.station_id
    }
}

/// A railway segemnt. It can be single, so it allows just
/// one car at the time or double so it is possible to have one car
/// for direction.
#[derive(PartialEq, Debug)]
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

    fn make_location(&self, dir: LineDirection, index: usize) -> car::CarLocation {
        self.choose_segment_info(dir).make_location(index)
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
#[derive(PartialEq, Debug)]
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

    fn make_location(&self, index: usize) -> car::CarLocation {
        match self.kind {
            SegmentType::Station(i) | SegmentType::Terminus(i) => {
                car::CarLocation::station(index, i)
            }
            SegmentType::Line => car::CarLocation::segment(index),
        }
    }
}

/// A railway segment can be a station,
/// a line connecting stations or a terminus (station
/// where a train must change direction)
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum SegmentType {
    Station(usize),
    Terminus(usize),
    Line,
}

/// A segment can be free or occupied
#[derive(PartialEq, Debug)]
enum SegmentStatus {
    Free,
    Occupied,
}

impl Default for SegmentStatus {
    fn default() -> Self {
        Self::Free
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_terminus_can_start() {
        // 4 train in depo, 3 steps between
        // each new train
        let mut term = Terminus::new(0, 4, 3);
        for _ in 0..4 {
            assert!(!term.can_start_new_train());
            term.step();

            assert!(!term.can_start_new_train());
            term.step();

            assert!(!term.can_start_new_train());
            term.step();

            assert!(term.can_start_new_train());
            term.step();
            term.add_new_train();
        }

        assert!(!term.can_start_new_train());
        term.step();
        assert!(!term.can_start_new_train());
        term.step();
        assert!(!term.can_start_new_train());
        term.step();
        assert!(!term.can_start_new_train());
        term.step();
        assert!(!term.can_start_new_train());
        term.step();
        assert!(!term.can_start_new_train());
        term.step();
    }

    #[test]
    fn text_next_step_railway() {
        let mut railway = init_railway();
        assert!(railway.next_step(0, LineDirection::DirectionB).is_none());
        assert!(railway.next_step(1, LineDirection::DirectionB).is_none());
        let res = railway.next_step(1, LineDirection::DirectionA);
        assert! {
            matches!{res, Some(NextStepInfo{kind, time, loc})
                if kind == SegmentType::Line &&
                    time == 0 &&
                    matches!{loc, car::CarLocation::Segment{index}
                        if index == 0
                    }
            }
        }
    }

    #[test]
    fn test_get_terminus() {
        let railway = init_railway();
        assert_eq!(
            *railway.get_terminus(LineDirection::DirectionA),
            Segment::Single(SegmentInfo {
                stat: SegmentStatus::Free,
                kind: SegmentType::Line,
                duration: 0
            })
        );
        assert_eq!(
            *railway.get_terminus(LineDirection::DirectionB),
            Segment::Single(SegmentInfo {
                stat: SegmentStatus::Occupied,
                kind: SegmentType::Line,
                duration: 0
            })
        );
    }

    #[test]
    fn test_check_next_railway() {
        let railway = init_railway();
        assert!(railway.is_free(1, LineDirection::DirectionA));
        assert!(!railway.is_free(1, LineDirection::DirectionB));
    }

    #[test]
    fn test_update_railway_position() {
        let mut railway = init_railway();
        let NextStepInfo { kind, time, loc } =
            railway.update_car_location(1, LineDirection::DirectionA);
        assert_eq!(time, 0);
        assert!(matches! {kind, SegmentType::Line});

        assert!(railway.is_free(0, LineDirection::DirectionB));
        assert!(!railway.is_free(1, LineDirection::DirectionA));
        assert!(!railway.is_free(1, LineDirection::DirectionB));
        assert!(railway.is_free(2, LineDirection::DirectionA));
        assert!(matches! {loc, car::CarLocation::Segment{index} if index == 0});
    }

    #[test]
    fn test_check_free_segment() {
        let single_segment = Segment::Single(init_segment_info(SegmentStatus::Occupied));
        assert!(!single_segment.is_free(LineDirection::DirectionA));
        assert!(!single_segment.is_free(LineDirection::DirectionB));

        let single_segment = Segment::Single(init_segment_info(SegmentStatus::Free));
        assert!(single_segment.is_free(LineDirection::DirectionA));
        assert!(single_segment.is_free(LineDirection::DirectionB));

        let double_segment = Segment::Double(
            init_segment_info(SegmentStatus::Free),
            init_segment_info(SegmentStatus::Occupied),
        );
        assert!(double_segment.is_free(LineDirection::DirectionA));
        assert!(!double_segment.is_free(LineDirection::DirectionB));

        let double_segment = Segment::Double(
            init_segment_info(SegmentStatus::Occupied),
            init_segment_info(SegmentStatus::Free),
        );
        assert!(!double_segment.is_free(LineDirection::DirectionA));
        assert!(double_segment.is_free(LineDirection::DirectionB));
    }

    #[test]
    fn test_set_free_segment() {
        let mut single_segment = Segment::Single(init_segment_info(SegmentStatus::Occupied));
        single_segment.set_free(LineDirection::DirectionA);
        assert!(single_segment.is_free(LineDirection::DirectionA));
        assert!(single_segment.is_free(LineDirection::DirectionB));

        let mut double_segment = Segment::Double(
            init_segment_info(SegmentStatus::Occupied),
            init_segment_info(SegmentStatus::Free),
        );

        double_segment.set_free(LineDirection::DirectionA);
        assert!(double_segment.is_free(LineDirection::DirectionA));
        assert!(double_segment.is_free(LineDirection::DirectionB));
    }

    #[test]
    fn test_set_occupied_segment() {
        let mut single_segment = Segment::Single(init_segment_info(SegmentStatus::Occupied));
        single_segment.set_occupied(LineDirection::DirectionA);
        assert!(!single_segment.is_free(LineDirection::DirectionA));
        assert!(!single_segment.is_free(LineDirection::DirectionB));

        let mut double_segment = Segment::Double(
            init_segment_info(SegmentStatus::Occupied),
            init_segment_info(SegmentStatus::Free),
        );

        double_segment.set_occupied(LineDirection::DirectionB);
        assert!(!double_segment.is_free(LineDirection::DirectionA));
        assert!(!double_segment.is_free(LineDirection::DirectionB));
    }

    fn init_segment_info(stat: SegmentStatus) -> SegmentInfo {
        SegmentInfo {
            kind: SegmentType::Line,
            stat,
            duration: 0,
        }
    }

    fn init_railway() -> Railway {
        let line = vec![
            Segment::Single(init_segment_info(SegmentStatus::Free)),
            Segment::Single(init_segment_info(SegmentStatus::Occupied)),
            Segment::Single(init_segment_info(SegmentStatus::Occupied)),
        ];
        Railway { line }
    }
}
