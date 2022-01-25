use crate::line;
use crate::passenger::{callbacks, PassengerFactory};
use crate::station::Station;
use crate::traffic_generator::TrafficGenerator;

pub fn engine<
    Tg: TrafficGenerator,
    Tc: callbacks::PassengerAction,
    Tf: callbacks::PassengerActionFactory<Tc> + Send + Sync,
>(
    steps: u32,
    passenger_factory: &PassengerFactory<Tg>,
    mut stations: Vec<Station<Tc>>,
    mut lines: Vec<line::Line<Tc>>,
    mut tf: Tf,
) {
    for step in 0..steps {
        passenger_factory.generate_traffic(step, &mut stations, &mut tf);
        move_trains(&mut lines);
        passenger_boarding(&mut lines, &mut stations);
    }
}

fn move_trains<Tc: callbacks::PassengerAction>(lines: &mut [line::Line<Tc>]) {
    lines.iter_mut().for_each(line::Line::step);
}

fn passenger_boarding<Tc: callbacks::PassengerAction>(
    lines: &mut [line::Line<Tc>],
    stations: &mut [Station<Tc>],
) {
    lines
        .iter_mut()
        .for_each(|ln| ln.boarding_operations(stations));
}
