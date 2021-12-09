use crate::line;
use crate::passenger::PassengerFactory;
use crate::station::Station;
use crate::traffic_generator::TrafficGenerator;

pub fn engine<Tg: TrafficGenerator>(
    steps: u32,
    passenger_factory: PassengerFactory<Tg>,
    mut stations: Vec<Station>,
    mut lines: Vec<line::Line>,
) {
    for step in 0..steps {
        passenger_factory.generate_traffic(step, &mut stations);
        move_trains(&mut lines);
        passenger_boarding(&mut lines, &mut stations);
    }
}

fn move_trains(lines: &mut [line::Line]) {
    lines.iter_mut().for_each(|ln| ln.step())
}

fn passenger_boarding(lines: &mut [line::Line], stations: &mut [Station]) {
    lines
        .iter_mut()
        .for_each(|ln| ln.boarding_operations(stations));
}
