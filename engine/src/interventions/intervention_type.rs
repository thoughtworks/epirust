pub trait InterventionType {
    fn name(&self) -> String;

    fn json_data(&self) -> String;
}
