/*
 * EpiRust
 * Copyright (c) 2020  ThoughtWorks, Inc.
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

use crate::models::custom_types::Count;

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Copy, Clone)]
pub struct StartingInfections {
    infected_mild_asymptomatic: Count,
    infected_mild_symptomatic: Count,
    infected_severe: Count,
    exposed: Count,
}

impl StartingInfections {
    pub fn new(mild_asymp: Count, mild_symp: Count, severe: Count, exposed: Count) -> StartingInfections {
        StartingInfections {
            infected_mild_asymptomatic: mild_asymp,
            infected_mild_symptomatic: mild_symp,
            infected_severe: severe,
            exposed,
        }
    }

    pub fn total(&self) -> Count {
        self.total_infected() + self.exposed
    }

    pub fn total_infected(&self) -> Count {
        self.get_infected_mild_asymptomatic() + self.get_infected_mild_symptomatic() + self.get_infected_severe()
    }

    pub fn get_infected_mild_asymptomatic(&self) -> Count {
        self.infected_mild_asymptomatic
    }

    pub fn get_infected_mild_symptomatic(&self) -> Count {
        self.infected_mild_symptomatic
    }

    pub fn get_infected_severe(&self) -> Count {
        self.infected_severe
    }

    pub fn get_exposed(&self) -> Count {
        self.exposed
    }
}

impl Default for StartingInfections {
    fn default() -> Self {
        StartingInfections { infected_mild_asymptomatic: 0, infected_mild_symptomatic: 0, infected_severe: 0, exposed: 1 }
    }
}

#[cfg(test)]
mod test {
    use crate::config::StartingInfections;
    use crate::models::custom_types::Count;
    use crate::utils::test_utils;

    #[test]
    fn test_default() {
        let actual_res = StartingInfections::default();
        assert_eq!(actual_res.infected_mild_symptomatic, 0);
        assert_eq!(actual_res.infected_mild_asymptomatic, 0);
        assert_eq!(actual_res.infected_severe, 0);
        assert_eq!(actual_res.exposed, 1);
    }

    #[test]
    fn test_getters() {
        let infected_mild_asymptomatic: Count = test_utils::random_number(0, 100000);
        let infected_mild_symptomatic: Count = test_utils::random_number(0, 100000);
        let infected_severe: Count = test_utils::random_number(0, 100000);
        let exposed: Count = test_utils::random_number(0, 100000);
        let infections = StartingInfections::new(infected_mild_asymptomatic, infected_mild_symptomatic, infected_severe, exposed);
        assert_eq!(infections.get_infected_mild_asymptomatic(), infected_mild_asymptomatic);
        assert_eq!(infections.get_infected_mild_symptomatic(), infected_mild_symptomatic);
        assert_eq!(infections.get_infected_severe(), infected_severe);
        assert_eq!(infections.get_exposed(), exposed);

        let expected_total_infected: Count = infected_mild_asymptomatic + infected_mild_symptomatic + infected_severe;
        assert_eq!(infections.total_infected(), expected_total_infected);

        assert_eq!(infections.total(), expected_total_infected + exposed)
    }
}
