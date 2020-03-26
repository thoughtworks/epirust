use crate::agent::{Citizen, State};
use crate::geography::Point;

#[derive(Serialize)]
pub struct CitizenState {
    citizen_id: i32,
    state: String,
    location: Point,
}

impl CitizenState {
    fn state_str(state: State) -> String {
        match state {
            State::Susceptible { .. } => "s".to_string(),
            State::Infected { .. } => "i".to_string(),
            State::Recovered { .. } => "r".to_string(),
            State::Deceased { .. } => "d".to_string(),
        }
    }
}

#[derive(Serialize)]
pub struct CitizenStatesAtHr {
    pub hr: i32,
    pub citizen_states: Vec<CitizenState>,
}

impl CitizenStatesAtHr {
    pub fn init(population_size: usize) -> CitizenStatesAtHr {
        CitizenStatesAtHr {
            hr: 1,
            citizen_states: Vec::with_capacity(population_size),
        }
    }

    pub fn update(&mut self, citizen: &Citizen, location: &Point) {
        self.citizen_states.push(CitizenState {
            citizen_id: citizen.id,
            state: CitizenState::state_str(citizen.state_machine.state),
            location: *location,
        })
    }

    pub fn next_hour(&mut self) {
        self.hr += 1;
        self.citizen_states.clear();
    }
}


#[cfg(test)]
mod tests {
    use crate::geography::Area;
    use crate::random_wrapper::RandomWrapper;

    use super::*;

    #[test]
    fn citizen_states_at_hr() {
        let mut citizen_states_at_hr = CitizenStatesAtHr::init(10);
        assert_eq!(citizen_states_at_hr.hr, 1);
        assert_eq!(citizen_states_at_hr.citizen_states.len(), 0);

        citizen_states_at_hr.update(&citizen(), &Point::new(1, 1));
        assert_eq!(citizen_states_at_hr.hr, 1);
        assert_eq!(citizen_states_at_hr.citizen_states.len(), 1);

        citizen_states_at_hr.next_hour();
        assert_eq!(citizen_states_at_hr.hr, 2);
        assert_eq!(citizen_states_at_hr.citizen_states.len(), 0);
    }

    fn citizen() -> Citizen {
        let area = Area::new(Point::new(0, 0), Point::new(1, 1));
        Citizen::new_citizen(1, area, area, Point::new(2, 2), true, true, &mut RandomWrapper::new())
    }
}