#  EpiRust
#  Copyright (c) 2020  ThoughtWorks, Inc.
#
#  This program is free software: you can redistribute it and/or modify
#  it under the terms of the GNU Affero General Public License as published by
#  the Free Software Foundation, either version 3 of the License, or
#  (at your option) any later version.
#
#  This program is distributed in the hope that it will be useful,
#  but WITHOUT ANY WARRANTY; without even the implied warranty of
#  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
#  GNU Affero General Public License for more details.
#
#  You should have received a copy of the GNU Affero General Public License
#  along with this program.  If not, see <http://www.gnu.org/licenses/>.
#

import json
import sys
import copy

sample = """
{
    "engine_id": "engine1",
    "config": {
      "sim_id": "sim-timestamp",
      "population": {
        "Auto": {
          "number_of_agents": 10000,
          "public_transport_percentage": 0.2,
          "working_percentage": 0.7
        }
      },
      "disease": {
        "regular_transmission_start_day": 5,
        "high_transmission_start_day": 20,
        "last_day": 40,
        "regular_transmission_rate": 0.025,
        "high_transmission_rate": 0.25,
        "death_rate": 0.035
      },
      "grid_size": 250,
      "hours": 10000,
      "interventions": [
        {
          "Vaccinate": {
            "at_hour": 5000,
            "percent": 0.2
          }
        },
        {
          "Lockdown": {
            "at_number_of_infections": 100,
            "essential_workers_population": 0.1,
            "lock_down_period": 21
          }
        }
      ]
    }
  }
"""

def generate_travel_matrix(region_count, travel_count):
    rows, cols = (region_count, region_count)
    arr = [[int(travel_count) for i in range(cols)] for j in range(rows)]
    for (i,row) in enumerate(arr):
        for (j,value) in enumerate(row):
            if i == j:
                arr[i][j] = 0
    return arr

def engine_names(engine_count):
    engine_names = []
    for i in range(engine_count):
        engine_names.append("engine" + str(i + 1))
    return engine_names

if __name__ == "__main__":
    if len(sys.argv) != 3:
        print("Usage: generate.py [n_engines] [population]")
        sys.exit(1)
    engines = int(sys.argv[1])
    population = int(sys.argv[2])
    json_sample = json.loads(sample)
    grid_size = 250
    if population == 10000:
        grid_size = 250
    elif population == 100000:
        grid_size = 800
    elif population == 1000000:
        grid_size = 2500
    elif population == 2000000:
        grid_size = 3550
    elif population == 5000000:
        grid_size = 5660
    else:
        print("Unsupported population size")
        sys.exit(1)
    travel_percent = 0.001
    travel_count = travel_percent * population
    json_sample["config"]["population"]["Auto"]["number_of_agents"] = population
    json_sample["config"]["grid_size"] = grid_size
    simulation_config = []
    for i in range(engines):
        json_sample["engine_id"] = "engine" + str(i + 1)
        simulation_config.append(copy.deepcopy(json_sample))

    travel_plan = { "regions": engine_names(engines), "matrix": generate_travel_matrix(engines, travel_count) }

    final = { "simulation": simulation_config, "travel_plan": travel_plan }

    with open("generated.json", "w") as outfile:
        outfile.write(json.dumps(final))
