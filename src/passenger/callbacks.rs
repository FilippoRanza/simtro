pub trait PassengerAction: Sync + Send + std::fmt::Debug {
    fn enter_station(&mut self, _: usize) {}
    fn leave_train(&mut self, _: usize) {}
}

pub trait PassengerActionFactory<T>
where
    T: PassengerAction,
{
    fn factory(&mut self) -> T;
}

impl PassengerAction for () {}
impl PassengerActionFactory<()> for () {
    fn factory(&mut self) -> () {
        ()
    }
}
