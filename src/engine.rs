use crate::passenger::PassengerFactory;
use crate::traffic_generator::TrafficGenerator;
use crate::utils::unique_id::UniqueId;
use crate::station::Station;


pub fn engine<Tg: TrafficGenerator>(steps: u32, tg: Tg, mut stations: Vec<Station>) {
    for step in 0..steps {
        generate_passengers(step, &tg, &mut stations);
        move_trains();
        board_passengers();
    }
}

fn generate_passengers<Tg: TrafficGenerator>(step: u32, tg: &Tg, sts: &mut [Station]) {
    let matrix = tg.next_traffic_matrix(step);
    let mut uid_gen = UniqueId::new();
    let pass_factory = PassengerFactory::new(matrix);
    for (stat, pass_iter) in sts.iter_mut().zip(pass_factory) {
        for p in uid_gen.set_id_iter(pass_iter) {
            stat.enter_passenger(p);
        }
    }
}

fn move_trains() {
    todo! {}
}

fn board_passengers() {
    todo! {}
}
