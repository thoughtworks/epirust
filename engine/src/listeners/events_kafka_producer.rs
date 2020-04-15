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


use std::any::Any;

use rdkafka::ClientConfig;
use rdkafka::producer::{FutureProducer, FutureRecord};

use crate::agent::Citizen;
use crate::geography::{Grid, Point};
use crate::listeners::events::citizen_state::CitizenStatesAtHr;
use crate::listeners::events::counts::Counts;
use crate::listeners::listener::Listener;
use crate::environment;
use crate::interventions::intervention_type::InterventionType;

pub struct EventsKafkaProducer {
    sim_id: String,
    producer: FutureProducer,
    citizen_states_buffer: CitizenStatesAtHr,
    enable_citizen_state_messages: bool,
    count_updated_topic: String,
    citizen_states_topic: String,
}

impl EventsKafkaProducer {
    pub fn new(sim_id: String, population_size: usize, enable_citizen_state_messages: bool) -> EventsKafkaProducer {
        let count_updated_topic = "counts_updated".to_string();
        let citizen_states_topic = "citizen_states_updated".to_string();
        let kafka_url = environment::kafka_url();
        EventsKafkaProducer {
            sim_id,
            producer: ClientConfig::new()
                .set("bootstrap.servers", kafka_url.as_str())
                .create()
                .expect("Could not create Kafka Producer"),
            citizen_states_buffer: CitizenStatesAtHr::init(population_size),
            enable_citizen_state_messages,
            count_updated_topic,
            citizen_states_topic,
        }
    }
}

impl Listener for EventsKafkaProducer {
    fn counts_updated(&mut self, counts: Counts) {
        let message = serde_json::to_string(&counts).expect("Failed to serialize counts");
        let record: FutureRecord<String, String> = FutureRecord::to(&self.count_updated_topic)
            .key(&self.sim_id)
            .payload(&message);
        self.producer.send(record, 0);
    }

    fn simulation_ended(&mut self) {
        let message = r#"{"simulation_ended": true}"#.to_string();
        let record: FutureRecord<String, String> = FutureRecord::to(&self.count_updated_topic)
            .key(&self.sim_id)
            .payload(&message);
        self.producer.send(record, 0);

        if self.enable_citizen_state_messages {
            let record2: FutureRecord<String, String> = FutureRecord::to(&self.citizen_states_topic)
                .key(&self.sim_id)
                .payload(&message);
            self.producer.send(record2, 0);
        }
    }

    fn citizen_state_updated(&mut self, hr: i32, citizen: &Citizen, location: &Point) {
        if !self.enable_citizen_state_messages {
            return;
        }
        if self.citizen_states_buffer.hr < hr {
            //hour incremented, push out all states to kafka
            let message = serde_json::to_string(&self.citizen_states_buffer)
                .expect("Failed to serialize citizen states");
            let record: FutureRecord<String, String> = FutureRecord::to(&self.citizen_states_topic)
                .key(&self.sim_id)
                .payload(&message);
            self.producer.send(record, 0);
            self.citizen_states_buffer.next_hour();
        } else {
            self.citizen_states_buffer.update(citizen, location);
        }
    }

    fn grid_updated(&self, grid: &Grid) {
        if self.enable_citizen_state_messages {
            let message = serde_json::to_string(grid);
            match message {
                Ok(m) => {
                    let record: FutureRecord<String, String> = FutureRecord::to(&self.citizen_states_topic)
                        .key(&self.sim_id)
                        .payload(&m);

                    self.producer.send(record, 0);
                }
                Err(e) => error!("Failed to parse the grid, cannot publish to kafka! Error: {}", e)
            }
        }
    }

    fn intervention_applied(&self,
                            _at_hour: i32,
                            _intervention: &dyn InterventionType,
    ) {
        let formatted_message = format!(r#"{{"hour": {}, "intervention": "{}", "data": {}}}"#,
                                        _at_hour,
                                        _intervention.name(),
                                        _intervention.json_data());

        let record: FutureRecord<String, String> = FutureRecord::to(&self.count_updated_topic)
            .key(&self.sim_id)
            .payload(&formatted_message);
        self.producer.send(record, 0);
    }


    fn as_any(&self) -> &dyn Any {
        self
    }
}
