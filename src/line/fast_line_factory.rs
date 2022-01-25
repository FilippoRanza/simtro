use super::line_factory;
use super::Duration;
use super::StationID;
use crate::passenger::callbacks;

pub struct FastLineFactoryConfig<Is, It> {
    station_ids: Is,
    station_time: Duration,
    line_len: It,
    split_len: Duration,
    depo_size: usize,
    train_delay: usize,
}

impl<Is, It> FastLineFactoryConfig<Is, It> {
    pub fn new(
        station_ids: Is,
        station_time: Duration,
        line_len: It,
        split_len: Duration,
        depo_size: usize,
        train_delay: usize,
    ) -> Self {
        Self {
            station_ids,
            station_time,
            line_len,
            split_len,
            depo_size,
            train_delay,
        }
    }
}

pub fn fast_line_factory<Is, It, Tc>(
    conf: FastLineFactoryConfig<Is, It>,
    total_station_count: usize,
) -> super::Line<Tc>
where
    Is: IntoIterator<Item = StationID>,
    It: IntoIterator<Item = Duration>,
    Tc: callbacks::PassengerAction,
{
    let lfc = build_line_factory_config(conf, total_station_count);
    line_factory::line_factory(lfc)
}

fn build_line_factory_config<Is, It>(
    conf: FastLineFactoryConfig<Is, It>,
    tsc: usize,
) -> line_factory::LineFactoryConfig
where
    Is: IntoIterator<Item = StationID>,
    It: IntoIterator<Item = Duration>,
{
    let station_info_iter = station_info_config_factory(conf.station_time, conf.station_ids);
    let line_info_iter = line_info_config_factory(conf.split_len, conf.line_len);
    line_factory::LineFactoryConfig::from_iter(station_info_iter, line_info_iter)
        .set_depo_size(conf.depo_size)
        .set_train_delay(conf.train_delay)
        .set_total_station_count(tsc)
}

fn station_info_config_factory<I>(
    time: Duration,
    iter: I,
) -> impl Iterator<Item = line_factory::StationInfoConfig>
where
    I: IntoIterator<Item = StationID>,
{
    iter.into_iter()
        .map(move |id| line_factory::StationInfoConfig::new(id, time))
}

fn line_info_config_factory<I>(
    split_line: Duration,
    iter: I,
) -> impl Iterator<Item = line_factory::LineInfoConfig>
where
    I: IntoIterator<Item = Duration>,
{
    iter.into_iter()
        .map(move |len| BuildLineChunkConfig::new(split_line, len).collect())
}

pub const CHUNK_COUNT: usize = 3;
struct BuildLineChunkConfig {
    count: usize,
    rem: usize,
    base: usize,
}

impl BuildLineChunkConfig {
    fn new(split_line: Duration, length: Duration) -> Self {
        if length < split_line {
            BuildLineChunkConfig {
                count: 1,
                rem: 0,
                base: length,
            }
        } else {
            let base = length / CHUNK_COUNT;
            let rem = length % CHUNK_COUNT;
            BuildLineChunkConfig {
                count: CHUNK_COUNT,
                rem,
                base,
            }
        }
    }
}

impl Iterator for BuildLineChunkConfig {
    type Item = line_factory::LineChunkConfig;
    fn next(&mut self) -> Option<Self::Item> {
        if self.count == 0 {
            None
        } else {
            self.count -= 1;
            let (len, kind) = if self.count % 2 == 1 {
                let len = self.base + self.rem;
                let kind = line_factory::LineChunkKind::Double;
                (len, kind)
            } else {
                (self.base, line_factory::LineChunkKind::Single)
            };
            let output = line_factory::LineChunkConfig::new(len, kind);
            Some(output)
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_build_line_chunk_config() {
        let mut blcc = BuildLineChunkConfig::new(6, 5);
        assert_eq!(
            blcc.next(),
            Some(line_factory::LineChunkConfig::new(
                5,
                line_factory::LineChunkKind::Single
            ))
        );
        assert_eq!(blcc.next(), None);

        let mut blcc = BuildLineChunkConfig::new(5, 5);
        assert_eq!(
            blcc.next(),
            Some(line_factory::LineChunkConfig::new(
                1,
                line_factory::LineChunkKind::Single
            ))
        );
        assert_eq!(
            blcc.next(),
            Some(line_factory::LineChunkConfig::new(
                3,
                line_factory::LineChunkKind::Double
            ))
        );
        assert_eq!(
            blcc.next(),
            Some(line_factory::LineChunkConfig::new(
                1,
                line_factory::LineChunkKind::Single
            ))
        );
        assert_eq!(blcc.next(), None);
    }
}
