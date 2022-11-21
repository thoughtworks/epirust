use crate::allocation_map::CitizenLocationMap;
use crate::citizen::Citizen;
use crate::geography::Point;
use crate::models::constants;
use crate::state_machine::{DiseaseHandler, Severity, State};
use common::disease::Disease;
use common::models::custom_types::{Day, Hour};
use common::utils::RandomWrapper;
use rand::prelude::SliceRandom;
use rand::Rng;

#[derive(Clone)]
pub struct DefaultDiseaseHandler;

impl DiseaseHandler for DefaultDiseaseHandler {
    fn on_infected(
        &self,
        sim_hr: Hour,
        infection_day: Day,
        severity: Severity,
        disease: &Disease,
        rng: &mut RandomWrapper,
    ) -> Option<State> {
        match severity {
            Severity::Pre { at_hour } if sim_hr - at_hour >= disease.get_pre_symptomatic_duration() => {
                let is_severe = rng.get().gen_bool(disease.get_percentage_severe_infected_population());
                let severity = if is_severe { Severity::Severe } else { Severity::Mild };
                Some(State::Infected { infection_day, severity })
            }
            _ => None,
        }
    }

    fn on_exposed(&self, at_hour: Hour, sim_hr: Hour, disease: &Disease, rng: &mut RandomWrapper) -> Option<State> {
        let option = constants::RANGE_FOR_EXPOSED.choose(rng.get());
        let random_factor = *option.unwrap();
        if sim_hr - at_hour >= (disease.get_exposed_duration() as i32 + random_factor) as Hour {
            let symptoms = rng.get().gen_bool(1.0 - disease.get_percentage_asymptomatic_population());
            let severity = if !symptoms { Severity::Asymptomatic } else { Severity::Pre { at_hour: sim_hr } };
            Some(State::Infected { infection_day: 0, severity })
        } else {
            None
        }
    }

    fn on_susceptible(
        &self,
        sim_hr: Hour,
        cell: Point,
        citizen: &Citizen,
        disease: &Disease,
        map: &CitizenLocationMap,
        rng: &mut RandomWrapper,
    ) -> Option<State> {
        if !citizen.work_quarantined && !citizen.is_vaccinated() {
            let neighbours = citizen.current_area.get_neighbors_of(cell);

            let neighbor_that_spreads_infection = neighbours
                .filter(|p| map.is_point_in_grid(p))
                .filter_map(|cell| map.get_agent_for(&cell))
                .filter(|agent| agent.state_machine.is_infected() && !agent.is_hospitalized())
                .find(|neighbor| rng.get().gen_bool(neighbor.get_infection_transmission_rate(disease)));

            if neighbor_that_spreads_infection.is_some() {
                return Some(State::Exposed { at_hour: sim_hr });
            }
        };
        None
    }

    fn on_routine_end(&self, current_state: &State, disease: &Disease, rng: &mut RandomWrapper) -> Option<State> {
        if let State::Infected { infection_day, severity } = current_state {
            match severity {
                Severity::Asymptomatic if *infection_day == constants::ASYMPTOMATIC_LAST_DAY => Some(State::Recovered),
                Severity::Mild if *infection_day == constants::MILD_INFECTED_LAST_DAY => Some(State::Recovered),
                Severity::Severe if *infection_day == disease.get_disease_last_day() => {
                    let state = if disease.is_to_be_deceased(rng) { State::Deceased } else { State::Recovered };
                    Some(state)
                }
                _ => None,
            }
        } else {
            None
        }
    }
}
