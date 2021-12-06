use crate::car::Car;
use crate::passenger::PassengerFactory;
use crate::station::Station;
use crate::traffic_generator::TrafficGenerator;
use crate::utils::unique_id::UniqueId;

pub fn engine<Tg: TrafficGenerator>(
    steps: u32,
    tg: Tg,
    mut stations: Vec<Station>,
    mut cars: Vec<Car>,
) {
    for step in 0..steps {
        generate_passengers(step, &tg, &mut stations);
        move_trains();
        board_passengers(&mut stations, &mut cars);
    }
}

fn generate_passengers<Tg: TrafficGenerator>(step: u32, tg: &Tg, sts: &mut [Station]) {
    let matrix = tg.next_traffic_flow(step);
    let mut uid_gen = UniqueId::new();
    //let pass_factory = PassengerFactory::new(matrix);
    //for (stat, pass_iter) in sts.iter_mut().zip(pass_factory) {
    //    for p in uid_gen.set_id_iter(pass_iter) {
    //        stat.enter_passenger(p);
    //    }
    //}
}

fn move_trains() {
    todo! {}
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
