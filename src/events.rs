#[derive(Serialize, Copy, Clone)]
pub struct Counts {
    hour: i32,
    susceptible: i32,
    infected: i32,
    quarantined: i32,
    recovered: i32,
    deceased: i32,
}

impl Counts {
    pub fn new(population: i32, infected: i32) -> Counts {
        Counts { hour: 0, susceptible: population, infected, quarantined: 0, recovered: 0, deceased: 0 }
    }

    pub fn get_infected(&self) -> i32 {
        self.infected
    }

    pub fn get_quarantined(&self) -> i32 {
        self.quarantined
    }

    pub fn get_hour(&self) -> i32 {
        self.hour
    }

    pub fn update_susceptible(&mut self, count: i32) {
        self.susceptible += count;
    }

    pub fn update_infected(&mut self, count: i32) {
        self.infected += count;
    }

    pub fn update_recovered(&mut self, count: i32) {
        self.recovered += count;
    }

    pub fn update_deceased(&mut self, count: i32) {
        self.deceased += count;
    }

    pub fn update_quarantined(&mut self, count: i32) {
        self.quarantined += count;
    }

    pub fn increment_hour(&mut self) {
        self.hour += 1;
    }
}
