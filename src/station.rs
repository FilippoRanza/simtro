
pub struct Station {
    state: StationState
}


pub enum StationState {
    Free,
    Car(usize)
}


pub struct Railway {
    length: usize
}
