pub trait InterventionType {
    fn name(&mut self) -> String;

    fn json_data(&mut self) -> String;
}
