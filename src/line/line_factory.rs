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

