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
use crate::kafka_consumer::KafkaConsumer;
use crate::kafka_producer::KafkaProducer;
use common::config::TravelPlanConfig;
use futures::StreamExt;
use rdkafka::error::KafkaError;
use rdkafka::message::BorrowedMessage;
use rdkafka::Message;
use std::collections::HashMap;
use std::error::Error;
use std::ops::Range;

const ROUTINE_TRAVEL_START_TIME: i64 = 7;
const ROUTINE_TRAVEL_END_TIME: i64 = 17;

//Note: these ticks are safe, they don't cause Lyme disease

pub async fn start_ticking(travel_plan: &TravelPlanConfig, hours: Range<i64>) {
    let mut acks: TickAcks = TickAcks::new(&travel_plan.get_regions());
    let mut producer = KafkaProducer::new();
    let consumer = KafkaConsumer::new();
    let mut message_stream = consumer.start_message_stream();
    let mut should_terminate = false;
    let is_commute_enabled = travel_plan.commute.enabled;
    let is_migration_enabled = travel_plan.migration.enabled;
    for h in hours {
        if !is_commute_enabled && (h % 24 == ROUTINE_TRAVEL_END_TIME || h % 24 == ROUTINE_TRAVEL_START_TIME) {
            continue;
        }
        if !is_migration_enabled && h % 24 == 0 {
            continue;
        }
        if h > 1 && h % 24 != 0 && h % 24 != ROUTINE_TRAVEL_START_TIME && h % 24 != ROUTINE_TRAVEL_END_TIME {
            continue;
        }
        acks.reset(h);
        let tick = Tick::new(h, should_terminate);

        let tick_ack_str = serde_json::to_string(&tick).unwrap();
        match producer.send_tick(&tick_ack_str) {
            Ok(_) => {
                if should_terminate {
                    break;
                }
                debug!("Sent tick successfully");
                while let Some(message) = message_stream.next().await {
                    let tick_ack = TickAck::parse_message(message);
                    match tick_ack {
                        Err(e) => {
                            error!(
                                "Received a message, but could not parse it.\n\
                                Error Details: {}",
                                e
                            )
                        }
                        Ok(ack) => {
                            debug!("Received tick acknowledgement successfully");
                            acks.push(ack);
                            if acks.all_received() {
                                should_terminate = acks.should_terminate();
                                break;
                            }
                        }
                    };
                }
            }
            Err(_) => {
                panic!("Failed to send simulation request to engines");
            }
        }
    }
}

#[derive(Debug, Serialize)]
pub struct Tick {
    hour: i64,
    terminate: bool,
}

impl Tick {
    pub fn new(hour: i64, terminate: bool) -> Tick {
        Tick { hour, terminate }
    }
}

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
pub struct TickAck {
    engine_id: String,
    hour: i64,
    counts: Counts,
}

impl TickAck {
    pub fn parse_message(message: Result<BorrowedMessage, KafkaError>) -> Result<TickAck, Box<dyn Error>> {
        let borrowed_message = message?;
        let parsed_message = borrowed_message.payload_view::<str>().unwrap()?;
        debug!("Received: {}", parsed_message);
        serde_json::from_str(parsed_message).map_err(|e| e.into())
    }
}

#[derive(Deserialize, Copy, Clone, Debug, PartialEq, Eq)]
pub struct Counts {
    hour: i32,
    susceptible: i32,
    exposed: i32,
    infected: i32,
    hospitalized: i32,
    recovered: i32,
    deceased: i32,
}

impl Counts {
    #[cfg(test)]
    pub fn new(hr: i32, s: i32, e: i32, i: i32, h: i32, r: i32, d: i32) -> Counts {
        Counts { hour: hr, susceptible: s, exposed: e, infected: i, hospitalized: h, recovered: r, deceased: d }
    }
}

/// stores a record of all the acks received for a tick
pub struct TickAcks {
    acks: HashMap<String, TickAck>,
    current_hour: i64,
    engines: Vec<String>,
}

impl TickAcks {
    pub fn new(engines: &[String]) -> TickAcks {
        TickAcks { acks: HashMap::new(), current_hour: 0, engines: engines.to_owned() }
    }

    pub fn reset(&mut self, h: i64) {
        self.current_hour = h;
        self.acks.clear();
    }

    pub fn push(&mut self, ack: TickAck) {
        if ack.hour != self.current_hour {
            error!("Received ack for another hour. Current hour: {}, received: {}", self.current_hour, ack.hour);
            return;
        }
        if self.acks.contains_key(&ack.engine_id) {
            error!("Received a duplicate ack for engine: {}", ack.engine_id);
            return;
        }
        if !self.engines.contains(&ack.engine_id) {
            error!("Received an ack from an unknown engine: {}", ack.engine_id);
            return;
        }
        let cloned = ack.clone();
        self.acks.insert(ack.engine_id, cloned);
    }

    pub fn all_received(&self) -> bool {
        self.acks.keys().count() == self.engines.len()
    }

    pub fn should_terminate(&self) -> bool {
        let total_exposed: i32 = self.acks.values().map(|ack| ack.counts.exposed).sum();
        let total_infected: i32 = self.acks.values().map(|ack| ack.counts.infected).sum();
        let total_hospitalized: i32 = self.acks.values().map(|ack| ack.counts.hospitalized).sum();
        total_exposed == 0 && total_infected == 0 && total_hospitalized == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_push_ack() {
        let engines = vec!["engine1".to_string(), "engine2".to_string()];
        let mut acks = TickAcks::new(&engines);
        acks.reset(22);
        let ack = TickAck { engine_id: "engine1".to_string(), hour: 22, counts: Counts::new(1, 100, 0, 0, 0, 0, 0) };
        acks.push(ack.clone());

        assert_eq!(*acks.acks.get("engine1").unwrap(), ack);
    }

    #[test]
    fn should_reset_current_hr() {
        let engines = vec!["engine1".to_string(), "engine2".to_string()];
        let mut acks = TickAcks::new(&engines);
        assert_eq!(acks.current_hour, 0);
        acks.reset(22);
        assert_eq!(acks.current_hour, 22);
    }

    #[test]
    fn should_terminate_when_exposed_and_infected_and_hospitalized_are_zero() {
        let mut acks = TickAcks::new(&vec!["engine1".to_string(), "engine2".to_string()]);
        acks.reset(1);
        acks.push(TickAck { engine_id: "engine1".to_string(), hour: 1, counts: Counts::new(1, 99, 0, 1, 0, 0, 0) });
        acks.push(TickAck { engine_id: "engine2".to_string(), hour: 1, counts: Counts::new(1, 99, 0, 1, 0, 0, 0) });
        assert!(!acks.should_terminate());

        acks.reset(2);
        acks.push(TickAck { engine_id: "engine1".to_string(), hour: 2, counts: Counts::new(2, 99, 0, 0, 1, 0, 0) });
        acks.push(TickAck { engine_id: "engine2".to_string(), hour: 2, counts: Counts::new(2, 100, 0, 0, 0, 0, 0) });
        assert!(!acks.should_terminate());

        acks.reset(3);
        acks.push(TickAck { engine_id: "engine1".to_string(), hour: 3, counts: Counts::new(2, 99, 1, 0, 0, 0, 0) });
        acks.push(TickAck { engine_id: "engine2".to_string(), hour: 3, counts: Counts::new(2, 100, 0, 0, 0, 0, 0) });
        assert!(!acks.should_terminate());

        acks.reset(4);
        acks.push(TickAck { engine_id: "engine1".to_string(), hour: 4, counts: Counts::new(3, 100, 0, 0, 0, 0, 0) });
        acks.push(TickAck { engine_id: "engine2".to_string(), hour: 4, counts: Counts::new(3, 100, 0, 0, 0, 0, 0) });
        assert!(acks.should_terminate());
    }

    // #[test]
    // #[should_panic(expected = "Received ack for another hour. Current hour: 0, received: 22")]
    // fn should_panic_if_recv_ack_for_another_hour() {
    //     let engines = vec!["engine1", "engine2"];
    //     let mut acks = TickAcks::new(engines);
    //     let ack = TickAck { engine_id: "engine1".to_string(), hour: 22 };
    //     acks.push(ack);
    // }
    //
    // #[test]
    // #[should_panic(expected = "Received a duplicate ack for engine: engine1")]
    // fn should_panic_if_recv_duplicate_ack() {
    //     let engines = vec!["engine1", "engine2"];
    //     let mut acks = TickAcks::new(engines);
    //     acks.reset(7);
    //     let ack1 = TickAck { engine_id: "engine1".to_string(), hour: 7 };
    //     acks.push(ack1);
    //     let ack2 = TickAck { engine_id: "engine1".to_string(), hour: 7 };
    //     acks.push(ack2);
    // }
    //
    // #[test]
    // #[should_panic(expected = "Received an ack from an unknown engine: engine_x")]
    // fn should_panic_if_recv_ack_from_unknown_engine() {
    //     let engines = vec!["engine1", "engine2"];
    //     let mut acks = TickAcks::new(engines);
    //     let ack = TickAck { engine_id: "engine_x".to_string(), hour: 0 };
    //     acks.push(ack);
    // }
}
