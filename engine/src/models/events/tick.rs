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

use common::models::custom_types::Hour;

#[derive(Debug, Copy, Deserialize, PartialEq, Eq, Clone)]
pub struct Tick {
    hour: Hour,
    terminate: bool,
}

impl Tick {
    pub fn hour(&self) -> Hour {
        self.hour
    }

    pub fn terminate(&self) -> bool {
        self.terminate
    }

    pub fn parse_tick(message: &str) -> Tick {
        serde_json::from_str(message).expect("Could not parse tick")
    }
}

#[cfg(test)]
mod tests {
    use crate::models::events::tick::Tick;

    #[test]
    fn should_parse_tick() {
        let json = r#"
        {
            "hour": 1,
            "terminate": false
        }"#;
        let expected = Tick { hour: 1, terminate: false };
        assert_eq!(expected, Tick::parse_tick(json));
    }
}
