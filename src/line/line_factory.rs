use super::Duration;
use super::StationID;
use super::line;
use crate::utils::counter;
use crate::utils::mixed_iterator;

pub struct LineFactoryConfig {
    station_duration: Vec<StationInfoConfig>,
    line_duration: Vec<LineInfoConfig>,
}

pub struct StationInfoConfig {
    index: StationID,
    duration: Duration,
}

pub struct LineInfoConfig {
    chunks: Vec<LineChunkConfig>,
}

pub struct LineChunkConfig {
    duration: Duration,
    kind: LineChunkKind,
}

pub enum LineChunkKind {
    Single,
    Double,
}

pub fn line_factory(config: LineFactoryConfig) -> super::Line {
    let (term_a, term_b) = get_terminus(&config.station_duration);
    let railway = railway_factory(config.station_duration, config.line_duration);

    todo! {}
}

fn get_terminus(sic: &[StationInfoConfig], ) -> (line::Terminus, line::Terminus) {
    todo!{}
}

fn railway_factory(sic: Vec<StationInfoConfig>, lic: Vec<LineInfoConfig>) -> line::Railway {
    let line = segment_vector_factory(sic, lic);
    line::Railway::new(line)
}

fn segment_vector_factory(
    sic: Vec<StationInfoConfig>,
    lic: Vec<LineInfoConfig>,
) -> Vec<line::Segment> {
    let line = Vec::with_capacity(sic.len() + lic.len());
    let mix_iter = mixed_iterator::MixingIterator::new(sic.into_iter(), lic.into_iter());
    mix_iter.fold(line, segment_factory)
}

fn segment_factory(
    mut line: Vec<line::Segment>,
    e: mixed_iterator::MixingIteratorItem<StationInfoConfig, LineInfoConfig>,
) -> Vec<line::Segment> {
    match e {
        mixed_iterator::MixingIteratorItem::T(sic) => {
            let tmp = station_segment_factory(sic);
            line.push(tmp);
        }
        mixed_iterator::MixingIteratorItem::K(lic) => {
            let mut tmp = rails_all_segment_factory(lic);
            line.append(&mut tmp);
        }
    };
    line
}

fn station_segment_factory(sic: StationInfoConfig) -> line::Segment {
    let dir_a = station_segment_info_factory(&sic);
    let dir_b = station_segment_info_factory(&sic);
    line::Segment::Double(dir_a, dir_b)
}

fn station_segment_info_factory(sic: &StationInfoConfig) -> line::SegmentInfo {
    let kind = line::SegmentType::Station(sic.index);
    line::SegmentInfo::new(kind, sic.duration)
}

fn rails_all_segment_factory(lic: LineInfoConfig) -> Vec<line::Segment> {
    lic.chunks.into_iter().map(rail_segment_factory).collect()
}

fn rail_segment_factory(chunk: LineChunkConfig) -> line::Segment {
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
    fn test_railway_factory() {
        let sic = (0..3)
            .map(|index| StationInfoConfig {
                index,
                duration: 10,
            })
            .collect();
        let lic = (0..2)
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
        let railway = segment_vector_factory(sic, lic);
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
