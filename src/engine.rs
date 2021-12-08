use crate::car::Car;
use crate::line;
use crate::passenger::PassengerFactory;
use crate::station::Station;
use crate::traffic_generator::TrafficGenerator;
use crate::utils::unique_id::UniqueId;

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
        .for_each(|ln| ln.passenger_boarding(stations));
}

fn board_passengers(sts: &mut [Station], cars: &mut [Car]) {
    unboard(cars);
    board(sts, cars);
}

fn board(sts: &mut [Station], cars: &mut [Car]) {
    for (i, s) in sts.iter_mut().enumerate() {
        for car in cars.iter_mut() {
            if car.at_station(i) {
                todo! {}
            }
        }
    }
}

fn unboard(cars: &mut [Car]) {
    // It is horrible but it can easily be parallelized with rayon
    cars.iter_mut()
        .filter(|c| c.in_station())
        .for_each(|c| c.unboard_passengers());
}
