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
use std::collections::HashMap;
use std::ops::Range;
use crate::kafka_producer::KafkaProducer;
use crate::kafka_consumer::KafkaConsumer;
use rdkafka::message::BorrowedMessage;
use rdkafka::error::KafkaError;
use std::error::Error;
use rdkafka::Message;
use futures::StreamExt;
use crate::travel_plan::TravelPlan;

//Note: these ticks are safe, they don't cause Lyme disease

pub async fn start_ticking(travel_plan: &TravelPlan, hours: Range<i32>) {
    let mut acks: TickAcks = TickAcks::new(travel_plan.get_regions());
    let mut producer = KafkaProducer::new();
    let consumer = KafkaConsumer::new();
    let mut message_stream = consumer.start_message_stream();
    let mut should_terminate = false;
    for h in hours {
        if h > 1 && h % 24 != 0 {
            continue;
        }
        acks.reset(h);
        let tick = Tick::new(h, travel_plan, should_terminate);

        match producer.send_tick(&tick).await.unwrap() {
            Ok(_) => {
                if should_terminate {
                    break;
                }
                while let Some(message) = message_stream.next().await {
                    let tick_ack = TickAck::parse_message(message);
                    match tick_ack {
                        Err(e) => {
                            error!("Received a message, but could not parse it.\n\
                                Error Details: {}", e)
                        }
                        Ok(ack) => {
                            acks.push(ack);
                            if acks.all_received() {
                                should_terminate = acks.should_terminate();
                                break;
                            }
                        }
                    };
                }
            }
            Err(_) => { panic!("Failed to send simulation request to engines"); }
        }
    }
}

#[derive(Debug, Serialize)]
pub struct Tick<'a> {
    hour: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    travel_plan: Option<&'a TravelPlan>,
    terminate: bool,
}

impl Tick<'_> {
    pub fn new(hour: i32, travel_plan: &TravelPlan, terminate: bool) -> Tick {
        let travel = if hour == 1 {
            Some(travel_plan)
        } else {
            None
        };
        return Tick {
            hour,
            travel_plan: travel,
            terminate
        };
    }
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct TickAck {
    engine_id: String,
    hour: i32,
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

#[derive(Deserialize, Copy, Clone, Debug, PartialEq)]
pub struct Counts {
    hour: i32,
    susceptible: i32,
    exposed: i32,
    infected: i32,
    quarantined: i32,
    recovered: i32,
    deceased: i32,
}

impl Counts {
    #[cfg(test)]
    pub fn new(hr: i32, s: i32, e: i32, i: i32, q: i32, r: i32, d: i32) -> Counts {
        Counts {
            hour: hr,
            susceptible: s,
            exposed: e,
            infected: i,
            quarantined: q,
            recovered: r,
            deceased: d,
        }
    }
}

/// stores a record of all the acks received for a tick
pub struct TickAcks {
    acks: HashMap<String, TickAck>,
    current_hour: i32,
    engines: Vec<String>,
}

impl TickAcks {
    pub fn new(engines: &Vec<String>) -> TickAcks {
        TickAcks {
            acks: HashMap::new(),
            current_hour: 0,
            engines: engines.clone(),
        }
    }

    pub fn reset(&mut self, h: i32) {
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
        let total_exposed: i32 = self.acks.values().map( |ack| ack.counts.exposed).sum();
        let total_infected: i32 = self.acks.values().map(|ack| ack.counts.infected).sum();
        let total_quarantined: i32 = self.acks.values().map(|ack| ack.counts.quarantined).sum();
        total_exposed ==0 && total_infected == 0 && total_quarantined == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;

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
    fn should_add_travel_payload_at_first_hour() {
        let config = Config::read("config/test/travel_plan.json").unwrap();
        let travel_plan = config.get_travel_plan();
        let tick = Tick::new(1, travel_plan, false);
        assert!(tick.travel_plan.is_some());

        let tick = Tick::new(2, &travel_plan, false);
        assert!(tick.travel_plan.is_none());
    }

    #[test]
    fn should_terminate_when_exposed_and_infected_and_quarantined_are_zero() {
        let mut acks = TickAcks::new(&vec!["engine1".to_string(), "engine2".to_string()]);
        acks.reset(1);
        acks.push(TickAck {
            engine_id: "engine1".to_string(),
            hour: 1,
            counts: Counts::new(1, 99, 0, 1, 0, 0, 0),
        });
        acks.push(TickAck {
            engine_id: "engine2".to_string(),
            hour: 1,
            counts: Counts::new(1, 99, 0, 1, 0, 0, 0),
        });
        assert!(!acks.should_terminate());

        acks.reset(2);
        acks.push(TickAck {
            engine_id: "engine1".to_string(),
            hour: 2,
            counts: Counts::new(2, 99, 0, 0, 1, 0, 0),
        });
        acks.push(TickAck {
            engine_id: "engine2".to_string(),
            hour: 2,
            counts: Counts::new(2, 100, 0, 0, 0, 0, 0),
        });
        assert!(!acks.should_terminate());

        acks.reset(3);
        acks.push(TickAck {
            engine_id: "engine1".to_string(),
            hour: 3,
            counts: Counts::new(2, 99, 1, 0, 0, 0, 0),
        });
        acks.push(TickAck {
            engine_id: "engine2".to_string(),
            hour: 3,
            counts: Counts::new(2, 100, 0, 0, 0, 0, 0),
        });
        assert!(!acks.should_terminate());

        acks.reset(4);
        acks.push(TickAck {
            engine_id: "engine1".to_string(),
            hour: 4,
            counts: Counts::new(3, 100, 0, 0, 0, 0, 0),
        });
        acks.push(TickAck {
            engine_id: "engine2".to_string(),
            hour: 4,
            counts: Counts::new(3, 100, 0, 0, 0, 0, 0),
        });
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
