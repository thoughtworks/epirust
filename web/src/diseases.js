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

export const diseases = {
    "small_pox": {
      "label": "Small Pox", 
      "regular_transmission_start_day": 10,
      "high_transmission_start_day": 16,
      "last_day": 22,
      "regular_transmission_rate": 0.05,
      "high_transmission_rate": 0.5,
      "death_rate": 0.2
    },
    "sars": {
      "label": "SARS", 
      "regular_transmission_start_day": 5,
      "high_transmission_start_day": 10,
      "last_day": 23,
      "regular_transmission_rate": 0.025,
      "high_transmission_rate": 0.25,
      "death_rate": 0.1
    },
    "covid_19": {
      "label": "COVID-19", 
      "regular_transmission_start_day": 5,
      "high_transmission_start_day": 20,
      "last_day": 40,
      "regular_transmission_rate": 0.025,
      "high_transmission_rate": 0.25,
      "death_rate": 0.2
    }
  }
