use super::line;
use super::Duration;
use super::StationID;
use crate::fleet;
use crate::utils::mixed_iterator;

pub struct LineFactoryConfig {
    station_duration: Vec<StationInfoConfig>,
    line_duration: Vec<LineInfoConfig>,
    total_staion_count: usize,
    depo_size: usize,
    train_delay: usize,
}

impl LineFactoryConfig {
    #[must_use]
    pub fn new(
        station_duration: Vec<StationInfoConfig>,
        line_duration: Vec<LineInfoConfig>,
        total_staion_count: usize,
        depo_size: usize,
        train_delay: usize,
    ) -> Self {
        Self {
            station_duration,
            line_duration,
            total_staion_count,

            depo_size,
            train_delay,
        }
    }

    #[must_use]
    pub fn from_iter<T, K>(station_duration: T, line_duration: K) -> Self
    where
        T: Iterator<Item = StationInfoConfig>,
        K: Iterator<Item = LineInfoConfig>,
    {
        Self::new(station_duration.collect(), line_duration.collect(), 0, 0, 0)
    }

    #[must_use]
    pub fn set_total_station_count(mut self, value: usize) -> Self {
        self.total_staion_count = value;
        self
    }

    #[must_use]
    pub fn set_depo_size(mut self, value: usize) -> Self {
        self.depo_size = value;
        self
    }

    #[must_use]
    pub fn set_train_delay(mut self, value: usize) -> Self {
        self.train_delay = value;
        self
    }
}

pub struct StationInfoConfig {
    index: StationID,
    duration: Duration,
    terminus: bool,
}

impl StationInfoConfig {
    #[must_use]
    pub fn new(index: StationID, duration: Duration) -> Self {
        Self {
            index,
            duration,
            terminus: false,
        }
    }

    fn set_terminus(&mut self) {
        self.terminus = true;
    }
}

pub struct LineInfoConfig {
    chunks: Vec<LineChunkConfig>,
}

impl LineInfoConfig {
    #[must_use]
    pub fn new(chunks: Vec<LineChunkConfig>) -> Self {
        Self { chunks }
    }
}

impl std::iter::FromIterator<LineChunkConfig> for LineInfoConfig {
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = LineChunkConfig>,
    {
        Self::new(iter.into_iter().collect())
    }
}

#[derive(Debug, PartialEq)]
pub struct LineChunkConfig {
    duration: Duration,
    kind: LineChunkKind,
}

impl LineChunkConfig {
    #[must_use]
    pub fn new(duration: Duration, kind: LineChunkKind) -> Self {
        Self { duration, kind }
    }
}

#[derive(Debug, PartialEq)]
pub enum LineChunkKind {
    Single,
    Double,
}

#[must_use]
pub fn line_factory(config: LineFactoryConfig) -> super::Line {
    let (term_a, term_b) = terminus_factory(
        &config.station_duration,
        config.depo_size,
        config.train_delay,
    );
    let railway = railway_factory(config.station_duration, config.line_duration);
    let train_count = 2 * config.depo_size;
    let fleet = fleet::Fleet::new(train_count);

    super::Line::new(
        train_count,
        term_a,
        term_b,
        railway,
        fleet,
        config.total_staion_count,
    )
}

fn terminus_factory(
    station_ics: &[StationInfoConfig],
    d: usize,
    t: usize,
) -> (line::Terminus, line::Terminus) {
    let term_a = build_terminus(station_ics.first().unwrap(), d, t);
    let term_b = build_terminus(station_ics.last().unwrap(), d, t);
    (term_a, term_b)
}

fn build_terminus(info: &StationInfoConfig, d: usize, t: usize) -> line::Terminus {
    let id = info.index;
    line::Terminus::new(id, d, t)
}

fn railway_factory(
    station_ics: Vec<StationInfoConfig>,
    line_ics: Vec<LineInfoConfig>,
) -> line::Railway {
    let station_ics = set_terminus(station_ics);
    let line = segment_vector_factory(station_ics, line_ics);
    line::Railway::new(line)
}

fn set_terminus(mut station_ics: Vec<StationInfoConfig>) -> Vec<StationInfoConfig> {
    station_ics.first_mut().unwrap().set_terminus();
    station_ics.last_mut().unwrap().set_terminus();
    station_ics
}

fn segment_vector_factory(
    station_ics: Vec<StationInfoConfig>,
    line_ics: Vec<LineInfoConfig>,
) -> Vec<line::Segment> {
    let line = Vec::with_capacity(station_ics.len() + line_ics.len());
    let mix_iter =
        mixed_iterator::MixingIterator::new(station_ics.into_iter(), line_ics.into_iter());
    mix_iter.fold(line, segment_factory)
}

fn segment_factory(
    mut line: Vec<line::Segment>,
    e: mixed_iterator::MixingIteratorItem<StationInfoConfig, LineInfoConfig>,
) -> Vec<line::Segment> {
    match e {
        mixed_iterator::MixingIteratorItem::T(station_ics) => {
            let tmp = station_segment_factory(&station_ics);
            line.push(tmp);
        }
        mixed_iterator::MixingIteratorItem::K(line_ics) => {
            let mut tmp = rails_all_segment_factory(&line_ics);
            line.append(&mut tmp);
        }
    };
    line
}

fn station_segment_factory(station_ic: &StationInfoConfig) -> line::Segment {
    let dir_a = station_segment_info_factory(station_ic);
    let dir_b = station_segment_info_factory(station_ic);
    line::Segment::Double(dir_a, dir_b)
}

fn station_segment_info_factory(station_ics: &StationInfoConfig) -> line::SegmentInfo {
    let kind = if station_ics.terminus {
        line::SegmentType::Terminus(station_ics.index)
    } else {
        line::SegmentType::Station(station_ics.index)
    };
    line::SegmentInfo::new(kind, station_ics.duration)
}

fn rails_all_segment_factory(line_ic: &LineInfoConfig) -> Vec<line::Segment> {
    line_ic.chunks.iter().map(rail_segment_factory).collect()
}

fn rail_segment_factory(chunk: &LineChunkConfig) -> line::Segment {
    let dur = chunk.duration;
    match chunk.kind {
        LineChunkKind::Single => line::Segment::Single(rail_segment_info_factory(dur)),
        LineChunkKind::Double => line::Segment::Double(
            rail_segment_info_factory(dur),
            rail_segment_info_factory(dur),
        ),
    }
}

fn rail_segment_info_factory(duration: Duration) -> line::SegmentInfo {
    line::SegmentInfo::new(line::SegmentType::Line, duration)
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_terminus_factory() {
        let station_ics: Vec<StationInfoConfig> = (0..4)
            .map(|index| StationInfoConfig {
                index,
                duration: 10,
                terminus: false,
            })
            .collect();

        let (ta, tb) = terminus_factory(&station_ics, 10, 4);
        let expect_ta = line::Terminus::new(0, 10, 4);
        let expect_tb = line::Terminus::new(3, 10, 4);
        assert_eq!(ta, expect_ta);
        assert_eq!(tb, expect_tb);
    }

    #[test]
    fn test_railway_factory() {
        let station_ics = (0..3)
            .map(|index| StationInfoConfig {
                index,
                duration: 10,
                terminus: false,
            })
            .collect();
        let line_ics = (0..2)
            .map(|i| i % 3 + 1)
            .map(|i| {
                (0..i)
                    .map(|j| LineChunkConfig {
                        duration: 10,
                        kind: if j % 2 == 0 {
                            LineChunkKind::Double
                        } else {
                            LineChunkKind::Single
                        },
                    })
                    .collect()
            })
            .map(|v| LineInfoConfig { chunks: v })
            .collect();
        let railway = segment_vector_factory(station_ics, line_ics);
        let expected = vec![
            line::Segment::Double(
                line::SegmentInfo::new(line::SegmentType::Station(0), 10),
                line::SegmentInfo::new(line::SegmentType::Station(0), 10),
            ),
            line::Segment::Double(
                line::SegmentInfo::new(line::SegmentType::Line, 10),
                line::SegmentInfo::new(line::SegmentType::Line, 10),
            ),
            line::Segment::Double(
                line::SegmentInfo::new(line::SegmentType::Station(1), 10),
                line::SegmentInfo::new(line::SegmentType::Station(1), 10),
            ),
            line::Segment::Double(
                line::SegmentInfo::new(line::SegmentType::Line, 10),
                line::SegmentInfo::new(line::SegmentType::Line, 10),
            ),
            line::Segment::Single(line::SegmentInfo::new(line::SegmentType::Line, 10)),
            line::Segment::Double(
                line::SegmentInfo::new(line::SegmentType::Station(2), 10),
                line::SegmentInfo::new(line::SegmentType::Station(2), 10),
            ),
        ];

        assert_eq!(railway, expected);
    }
}
