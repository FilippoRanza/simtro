use crate::passenger::PassengerFactory;
use crate::traffic_generator::TrafficGenerator;
use crate::utils::unique_id::UniqueId;

pub fn engine<Tg: TrafficGenerator>(steps: u32, tg: Tg) {
    for step in 0..steps {
        generate_passengers(step, &tg);
        move_trains();
        board_passengers();
    }
}

fn generate_passengers<Tg: TrafficGenerator>(step: u32, tg: &Tg) {
    let matrix = tg.next_traffic_matrix(step);
    let mut uid_gen = UniqueId::new();
    let pass_factory = PassengerFactory::new(matrix);
}

fn move_trains() {
    todo! {}
}

fn board_passengers() {
    todo! {}
}
