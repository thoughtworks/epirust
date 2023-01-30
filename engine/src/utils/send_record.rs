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

use rdkafka::error::KafkaError;
use rdkafka::producer::{BaseRecord, DefaultProducerContext, ThreadedProducer};
use std::time::Instant;

pub type SendResult<'a> = Result<(), (KafkaError, BaseRecord<'a, String, String>)>;

pub trait SendRecord {
    fn send_record(&self, record: BaseRecord<String, String>);
}

impl SendRecord for ThreadedProducer<DefaultProducerContext> {
    fn send_record(&self, record: BaseRecord<String, String>) {
        let msg = &*format!("Failed to send msg {:?}, Reason", record.payload);
        self.send(record).expect(msg);
    }
}
