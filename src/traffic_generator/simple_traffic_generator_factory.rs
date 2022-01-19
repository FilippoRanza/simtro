//! This module implements a factory to build a SimpleTrafficGenerator
//! matrix that is required by a PassengerGeneraror.

use super::simple_traffic_generator as stg;
use super::{Int, Node};

use rand::prelude::*;
use std::collections::HashSet;

pub struct SimpleTrafficGeneratorConfig {
    begin: Int,
    end: Int,
    resolution: Int,
    anchors: Vec<(Int, Node)>,
    min_anchor: Node,
    max_anchor: Node,
}

pub fn simple_traffic_generator_factory(
    traffic: Vec<Vec<Int>>,
    config: &SimpleTrafficGeneratorConfig,
) -> Vec<Vec<Option<stg::SimpleTrafficGenerator>>> {
    traffic
        .into_iter()
        .map(|row| row.into_iter().map(|t| make_stg(t, &config)).collect())
        .collect()
}

fn make_stg(t: Int, conf: &SimpleTrafficGeneratorConfig) -> Option<stg::SimpleTrafficGenerator> {
    if t > 0 {
        let conf = make_config(t, conf);
        Some(stg::SimpleTrafficGenerator::new(conf))
    } else {
        None
    }
}

fn make_config(t: Int, conf: &SimpleTrafficGeneratorConfig) -> stg::SimpleTrafficGeneratorConfig {
    let anchors = apply_noise(
        &conf.anchors,
        conf.begin,
        conf.end,
        conf.min_anchor,
        conf.max_anchor,
    );
    stg::SimpleTrafficGeneratorConfig {
        time_begin: conf.begin,
        time_end: conf.end,
        anchors,
        traffic: t,
        minute_resolution: conf.resolution,
    }
}

fn apply_noise(
    anchors: &[(Int, Node)],
    time_begin: Int,
    time_end: Int,
    min_anchor: Node,
    max_anchor: Node,
) -> Vec<(Int, Node)> {
    let mut selected_times = HashSet::new();
    let mut anchors: Vec<(Int, Node)> = anchors
        .iter()
        .map(|a| {
            remap_tuple(
                *a,
                time_begin,
                time_end,
                min_anchor,
                max_anchor,
                &mut selected_times,
            )
        })
        .collect();
    anchors.sort_by_key(|t| t.0);
    anchors
}

fn remap_tuple(
    a: (Int, Node),
    tb: Int,
    te: Int,
    min_a: Node,
    max_a: Node,
    selected: &mut HashSet<Int>,
) -> (Int, Node) {
    let (time, traffic) = a;
    let mut rng = thread_rng();
    let time = remap_time(&mut rng, tb, te, time, selected);
    let traffic = remap_value(&mut rng, min_a, max_a, traffic);
    (time, traffic)
}

fn remap_time(
    rng: &mut ThreadRng,
    min: Int,
    max: Int,
    time: Int,
    selected: &mut HashSet<Int>,
) -> Int {
    loop {
        let tmp = remap_value(rng, min as Node, max as Node, time as Node) as Int;
        if !selected.contains(&tmp) {
            selected.insert(tmp);
            return tmp;
        }
    }
}

fn remap_value(rng: &mut ThreadRng, min: Node, max: Node, mean: Node) -> Node {
    let normal = rand_distr::Normal::new(mean, 1.).unwrap();
    normal
        .sample_iter(rng)
        .filter(|n| *n >= min && *n <= max)
        .next()
        .unwrap()
}

#[cfg(test)]
mod test {

    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_apply_noise() {
        let anchors = vec![
            (5, 1.),
            (8, 10.),
            (12, 2.),
            (15, 3.),
            (17, 4.),
            (18, 4.),
            (20, 1.),
        ];

        let min_anchor = 1.;
        let max_anchor = 14.;

        let time_begin = 5;
        let time_end = 21;
        for _ in 0..10000 {
            let res = apply_noise(&anchors, time_begin, time_end, min_anchor, max_anchor);
            assert_eq!(res.len(), anchors.len());
            let mut prev: Option<Int> = None;
            for (t, a) in &res {
                assert!(*t >= time_begin && *t <= time_end);
                assert!(*a >= min_anchor && *a <= max_anchor);
                if let Some(prev) = prev {
                    assert!(prev < *t);
                }
                prev = Some(*t);
            }
        }
    }

    #[test]
    fn test_remap_value() {
        let mut rng = thread_rng();
        let min = 5.;
        let max = 15.;
        let mean = 10.;
        for _ in 0..10000 {
            let value = remap_value(&mut rng, min, max, mean);
            assert!(value >= min && value <= max, "{min} {max} {value}");
        }
    }

    #[test]
    fn test_remap_time() {
        let mut rng = thread_rng();
        let mut set = HashSet::new();
        let mut counter = HashMap::new();
        let min = 4;
        let max = 34;
        let mean = 23;

        for _ in 0..10 {
            let value = remap_time(&mut rng, min, max, mean, &mut set);
            assert!(value >= min && value <= max, "{min} {max} {value}");
            if let Some(count) = counter.get_mut(&value) {
                *count += 1
            } else {
                counter.insert(value, 1);
            }
        }

        for value in counter.values() {
            assert_eq!(*value, 1);
        }
    }
}
