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

use std::error::Error;

use futures::StreamExt;
use rdkafka::{ClientConfig, Message};
use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::error::KafkaError;
use rdkafka::message::BorrowedMessage;
use rdkafka::producer::{FutureProducer, FutureRecord};

use crate::epidemiology_simulation::Epidemiology;
use std::any::Any;
use crate::config::Config;
use crate::agent::{Citizen, State};
use crate::listeners::listener::Listener;
use crate::listeners::events::counts::Counts;
use crate::geography::Point;

pub struct KafkaProducer {
    producer: FutureProducer,
    citizen_states_buffer: CitizenStatesAtHr,
    enable_citizen_state_messages: bool,
}

impl KafkaProducer {
    pub fn new(population_size: usize, enable_citizen_state_messages: bool) -> KafkaProducer {
        KafkaProducer {
            producer: ClientConfig::new()
                .set("bootstrap.servers", "localhost:9092")
                .create()
                .expect("Could not create Kafka Producer"),
            citizen_states_buffer: CitizenStatesAtHr::init(population_size),
            enable_citizen_state_messages,
        }
    }
}

impl Listener for KafkaProducer {
    fn counts_updated(&mut self, counts: Counts) {
        let message = serde_json::to_string(&counts).expect("Failed to serialize counts");
        let record: FutureRecord<String, String> = FutureRecord::to("counts_updated").payload(&message);
        self.producer.send(record, 0);
    }

    fn simulation_ended(&mut self) {
        let message = r#"{"simulation_ended": true}"#.to_string();
        let record: FutureRecord<String, String> = FutureRecord::to("counts_updated")
            .payload(&message);
        self.producer.send(record, 0);
    }

    fn citizen_state_updated(&mut self, hr: i32, citizen: &Citizen, location: &Point) {
        if !self.enable_citizen_state_messages {
            return;
        }
        if self.citizen_states_buffer.hr < hr {
            //hour incremented, push out all states to kafka
            let message = serde_json::to_string(&self.citizen_states_buffer)
                .expect("Failed to serialize citizen states");
            let record: FutureRecord<String, String> = FutureRecord::to("citizen_states_updated")
                .payload(&message);
            self.producer.send(record, 0);
            self.citizen_states_buffer.next_hour();
        } else {
            self.citizen_states_buffer.update(citizen, location);
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Serialize)]
struct CitizenState {
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
struct CitizenStatesAtHr {
    hr: i32,
    citizen_states: Vec<CitizenState>,
}

impl CitizenStatesAtHr {
    fn init(population_size: usize) -> CitizenStatesAtHr {
        CitizenStatesAtHr {
            hr: 1,
            citizen_states: Vec::with_capacity(population_size),
        }
    }

    fn update(&mut self, citizen: &Citizen, location: &Point) {
        self.citizen_states.push(CitizenState {
            citizen_id: citizen.id,
            state: CitizenState::state_str(citizen.state_machine.state),
            location: *location,
        })
    }

    fn next_hour(&mut self) {
        self.hr += 1;
        self.citizen_states.clear();
    }
}

pub struct KafkaConsumer {
    consumer: StreamConsumer,
}

impl KafkaConsumer {
    pub fn new() -> KafkaConsumer {
        let consumer: StreamConsumer = ClientConfig::new()
            .set("bootstrap.servers", "localhost:9092")
            .set("group.id", "engine")
            .create()
            .expect("Consumer creation failed");

        let topics = ["simulation_requests"];
        consumer.subscribe(&topics)
            .expect("Can't subscribe to specified topics");

        return KafkaConsumer { consumer };
    }

    pub async fn listen_loop(&self) {
        let mut message_stream = self.consumer.start();
        while let Some(message) = message_stream.next().await {
            let simulation_config = self.parse_message(message);
            match simulation_config {
                Err(e) => {
                    println!("Received a message, but could not parse it.\n\
                        Error Details: {}", e)
                }
                Ok(config) => {
                    let mut epidemiology = Epidemiology::new(&config);
                    epidemiology.run(&config);
                }
            };
        }
    }

    fn parse_message(&self, message: Result<BorrowedMessage, KafkaError>) -> Result<Config, Box<dyn Error>> {
        let borrowed_message = message?;
        let parsed_message = borrowed_message.payload_view::<str>().unwrap()?;
        serde_json::from_str(parsed_message).map_err(|e| e.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geography::Area;
    use crate::random_wrapper::RandomWrapper;

    #[test]
    fn citizen_states_at_hr() {
        let mut citizen_states_at_hr = CitizenStatesAtHr::init(10);
        assert_eq!(citizen_states_at_hr.hr, 1);
        assert_eq!(citizen_states_at_hr.citizen_states.len(), 0);

        citizen_states_at_hr.update(&citizen(), &Point::new(1,1));
        assert_eq!(citizen_states_at_hr.hr, 1);
        assert_eq!(citizen_states_at_hr.citizen_states.len(), 1);

        citizen_states_at_hr.next_hour();
        assert_eq!(citizen_states_at_hr.hr, 2);
        assert_eq!(citizen_states_at_hr.citizen_states.len(), 0);
    }

    fn citizen() -> Citizen {
        let area = Area::new(Point::new(0,0), Point::new(1,1));
        Citizen::new_citizen(1, area, area, Point::new(2,2), true, true, &mut RandomWrapper::new())
    }
}
