/*
 * EpiRust
 * Copyright (c) 2022  ThoughtWorks, Inc.
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 *
 */

use common::config::{StartingInfections, TravelPlanConfig};
use common::models::CommutePlan;
use common::utils::RandomWrapper;
use rand::seq::IteratorRandom;
use rand::Rng;

use crate::citizen::work_status::WorkStatus;
use crate::citizen::{Citizen, CitizensData};
use crate::geography::Point;
use crate::helpers::string_to_s16;

pub fn citizen_factory(
    ctz_data: CitizensData,
    travel_plan_config: &Option<TravelPlanConfig>,
    rng: &mut RandomWrapper,
) -> Vec<Citizen> {
    let total_home_locations = ctz_data.home_locations.len();
    let mut agent_list = Vec::with_capacity(total_home_locations);

    let commute_plan = travel_plan_config.as_ref().filter(|t_conf| t_conf.commute.enabled).map(|t_conf| t_conf.commute_plan());

    let mut current_number_of_public_transport_users = 0;
    for i in 0..ctz_data.number_of_agents as usize {
        let agent = create_citizen(i, &ctz_data, rng, &mut current_number_of_public_transport_users);
        agent_list.push(agent);
    }
    debug!("all working agents: {}", agent_list.iter().filter(|a| { a.work_status != WorkStatus::NA }).count());
    debug!("agents with public transport percentage: {}", agent_list.iter().filter(|a| { a.uses_public_transport }).count());

    set_starting_infections(&mut agent_list, ctz_data.starting_infections, rng);

    if let Some(cp) = commute_plan {
        update_commuters(&mut agent_list, cp, ctz_data.region);
    }

    agent_list
}

fn create_citizen(
    number: usize,
    ctz_data: &CitizensData,
    rng: &mut RandomWrapper,
    current_number_of_public_transport_users: &mut usize,
) -> Citizen {
    let total_home_locations = ctz_data.home_locations.len();
    let total_work_locations = ctz_data.work_locations.len();
    let is_a_working_citizen = rng.get().gen_bool(ctz_data.working_percentage);

    let home_location = ctz_data.home_locations[(number % total_home_locations)].clone();
    let work_location = ctz_data.work_locations[(number % total_work_locations)].clone();

    let uses_public_transport = rng.get().gen_bool(ctz_data.public_transport_percentage)
        && is_a_working_citizen
        && *current_number_of_public_transport_users < ctz_data.public_transport_locations.len();

    let public_transport_location: Point = if uses_public_transport {
        ctz_data.public_transport_locations[*current_number_of_public_transport_users]
    } else {
        home_location.get_random_point(rng)
    };
    if uses_public_transport {
        *current_number_of_public_transport_users += 1
    };

    let work_location = if is_a_working_citizen { work_location } else { home_location.clone() };
    let work_status = Citizen::derive_work_status(is_a_working_citizen, rng);

    Citizen::new(home_location, work_location, public_transport_location, uses_public_transport, work_status, rng)
}

fn update_commuters(agent_list: &mut [Citizen], commute_plan: CommutePlan, self_region: String) {
    debug!("Start updating commuters");
    let total_commuters_by_region: Vec<(String, u32)> = commute_plan.get_total_commuters_by_region(self_region.clone());

    let mut working_agents = agent_list
        .iter_mut()
        .filter(|agent| {
            agent.is_working() && agent.work_location.location_id == self_region.clone() && agent.uses_public_transport
        })
        .take(total_commuters_by_region.iter().map(|(_, n)| *n as usize).sum());

    debug!("Got all working agents");

    for (region, commuters) in total_commuters_by_region {
        working_agents.by_ref().take(commuters as usize).for_each(|working_agent| {
            working_agent.work_location.location_id = string_to_s16(&region);
        });
        debug!("Updated {} commuters for region {}", region, commuters);
    }
    debug!("updated the commuters");
}

pub fn set_starting_infections(agent_list: &mut [Citizen], start_infections: &StartingInfections, rng: &mut RandomWrapper) {
    if start_infections.total() as usize > agent_list.len() {
        panic!("There are {} people set to infect, but only {} agents available", start_infections.total(), agent_list.len())
    }
    if start_infections.total() == 0 {
        warn!("Simulation configured to start without any infected agents");
    }
    let mut to_infect = agent_list.iter_mut().choose_multiple(rng.get(), start_infections.total() as usize);
    let mut citizens = to_infect.iter_mut();

    for _i in 0..start_infections.get_exposed() {
        citizens.next().unwrap().state_machine.expose(0);
    }
    for _i in 0..start_infections.get_infected_mild_asymptomatic() {
        citizens.next().unwrap().state_machine.set_mild_asymptomatic()
    }
    for _i in 0..start_infections.get_infected_mild_symptomatic() {
        citizens.next().unwrap().state_machine.set_mild_symptomatic()
    }
    for _i in 0..start_infections.get_infected_severe() {
        citizens.next().unwrap().state_machine.set_severe_infected()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::citizen::citizen_factory::{citizen_factory, set_starting_infections};
    use crate::geography::Area;

    fn before_each() -> Vec<Citizen> {
        let mut rng = RandomWrapper::new();
        let engine_id = "engine1".to_string();
        let home_locations = vec![
            Area::new(&engine_id, Point::new(0, 0), Point::new(2, 2)),
            Area::new(&engine_id, Point::new(3, 0), Point::new(4, 2)),
        ];

        let work_locations = vec![
            Area::new(&engine_id.clone(), Point::new(5, 0), Point::new(6, 2)),
            Area::new(&engine_id, Point::new(7, 0), Point::new(8, 2)),
        ];

        let public_transport_location = vec![Point::new(5, 0), Point::new(5, 1), Point::new(5, 2), Point::new(5, 3)];
        let start_infections = StartingInfections::new(0, 0, 0, 1);

        let ctz_data = CitizensData::new(
            "engine1".to_string(),
            4,
            &home_locations,
            &work_locations,
            &public_transport_location,
            0.5,
            0.5,
            &start_infections,
        );

        citizen_factory(ctz_data, &None, &mut rng)
    }

    #[test]
    fn generate_citizen() {
        let citizen_list = before_each();
        let engine_id = "engine1".to_string();
        let expected_home_locations = vec![
            Area::new(&engine_id, Point::new(0, 0), Point::new(2, 2)),
            Area::new(&engine_id, Point::new(3, 0), Point::new(4, 2)),
        ];

        assert_eq!(citizen_list.len(), 4);
        assert_eq!(citizen_list.iter().filter(|c| c.is_exposed()).count(), 1);

        for citizen in &citizen_list {
            assert!(expected_home_locations.contains(&citizen.home_location));
        }
    }

    #[test]
    fn should_set_starting_infections() {
        let engine_id = "engine1".to_string();
        let home_location = Area::new(&engine_id, Point::new(0, 0), Point::new(10, 10));
        let work_location = Area::new(&engine_id, Point::new(11, 0), Point::new(20, 20));
        let mut citizens = Vec::new();
        let mut rng = RandomWrapper::new();
        for _i in 0..20 {
            let citizen =
                Citizen::new(home_location.clone(), work_location.clone(), Point::new(2, 2), false, WorkStatus::Normal, &mut rng);
            citizens.push(citizen);
        }

        let start_infections = StartingInfections::new(2, 3, 4, 5);

        set_starting_infections(&mut citizens, &start_infections, &mut rng);

        let actual_exposed = citizens.iter().filter(|citizen| citizen.is_exposed()).count();
        let actual_mild_asymp = citizens.iter().filter(|citizen| citizen.is_mild_asymptomatic()).count();
        let actual_mild_symp = citizens.iter().filter(|citizen| citizen.is_mild_symptomatic()).count();
        let actual_severe = citizens.iter().filter(|citizen| citizen.is_infected_severe()).count();

        assert_eq!(2, actual_mild_asymp);
        assert_eq!(3, actual_mild_symp);
        assert_eq!(4, actual_severe);
        assert_eq!(5, actual_exposed);
    }
}
