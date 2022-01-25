use ndarray::arr2;
use simtro::*;

#[test]
fn test_run() {
    let traffic = vec![
        vec![0, 10, 5, 4, 3],
        vec![5, 0, 12, 3, 3],
        vec![5, 11, 0, 6, 3],
        vec![4, 4, 7, 0, 1],
        vec![4, 4, 7, 2, 0],
    ];

    let adj_mat = arr2(&[
        [u32::MAX, 1, u32::MAX, u32::MAX, u32::MAX],
        [1, u32::MAX, 1, 1, 1],
        [u32::MAX, 1, u32::MAX, u32::MAX, u32::MAX],
        [u32::MAX, 1, u32::MAX, u32::MAX, u32::MAX],
        [u32::MAX, 1, u32::MAX, u32::MAX, u32::MAX],
    ]);

    let total_station_count = adj_mat.ncols();
    let (_, dir, inter) = routes::build_directions(adj_mat, &[(0, 2), (3, 4)]);

    let begin = 5;
    let end = 22;
    let resolution = 2;
    let anchors = vec![];
    let min_anchor = 1.;
    let max_anchor = 10.;
    let steps = get_steps(begin, end, resolution);

    let stgc = traffic_generator::SimpleTrafficGeneratorConfig::new(
        (begin, end),
        resolution,
        anchors,
        (min_anchor, max_anchor),
    );
    let stg = traffic_generator::simple_traffic_generator_factory(traffic, &stgc);
    let pf = passenger::PassengerFactory::new(stg);

    let stations = station::station_list_factory(5, &dir, &inter);

    let fast_line_config =
        line::fast_line_factory::FastLineFactoryConfig::new(0..=2, 5, [6, 6], 7, 4, 11);
    let line_a = line::fast_line_factory::fast_line_factory(fast_line_config, total_station_count);
    let fast_line_config =
        line::fast_line_factory::FastLineFactoryConfig::new([3, 1, 4], 5, [6, 6], 7, 4, 11);
    let line_b = line::fast_line_factory::fast_line_factory(fast_line_config, total_station_count);
    let lines = vec![line_a, line_b];

    engine::engine(steps, &pf, stations, lines, ());
}
