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

//Note: these ticks are safe, they don't cause Lyme disease

#[derive(Debug, Deserialize)]
pub struct TickAck {
    engine_id: String,
    hour: i32,
}

/// stores a record of all the acks received for a tick
pub struct TickAcks<'a> {
    acks: HashMap<String, i32>,
    current_hour: i32,
    engines: Vec<&'a str>,
}

impl TickAcks<'_> {
    pub fn new(engines: Vec<&str>) -> TickAcks {
        TickAcks {
            acks: HashMap::new(),
            current_hour: 0,
            engines,
        }
    }

    pub fn reset(&mut self, h: i32) {
        self.current_hour = h;
        self.acks.clear();
    }

    pub fn push(&mut self, ack: TickAck) {
        if ack.hour != self.current_hour {
            println!("Received ack for another hour. Current hour: {}, received: {}", self.current_hour, ack.hour);
            return;
        }
        if self.acks.contains_key(&ack.engine_id) {
            println!("Received a duplicate ack for engine: {}", ack.engine_id);
            return;
        }
        if !self.engines.contains(&ack.engine_id.as_str()) {
            println!("Received an ack from an unknown engine: {}", ack.engine_id);
            return;
        }
        self.acks.insert(ack.engine_id, ack.hour);
    }

    pub fn all_received(&self) -> bool {
        self.acks.keys().count() == self.engines.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_push_ack() {
        let engines = vec!["engine1", "engine2"];
        let mut acks = TickAcks::new(engines);
        acks.reset(22);
        let ack = TickAck { engine_id: "engine1".to_string(), hour: 22 };
        acks.push(ack);

        assert_eq!(*acks.acks.get("engine1").unwrap(), 22 as i32);
    }

    #[test]
    fn should_reset_current_hr() {
        let engines = vec!["engine1", "engine2"];
        let mut acks = TickAcks::new(engines);
        assert_eq!(acks.current_hour, 0);
        acks.reset(22);
        assert_eq!(acks.current_hour, 22);
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
