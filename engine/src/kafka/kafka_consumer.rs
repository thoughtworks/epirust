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
use std::path::Path;

use futures::StreamExt;
use opentelemetry::trace::{FutureExt, TraceContextExt, Tracer};
use opentelemetry::{global, Context};
use rdkafka::consumer::Consumer;
use rdkafka::consumer::{MessageStream, StreamConsumer};
use rdkafka::error::KafkaError;
use rdkafka::message::BorrowedMessage;
use rdkafka::message::Message;
use rdkafka::ClientConfig;

use common::config::request::Request;

use crate::epidemiology_simulation::Epidemiology;
use crate::kafka::kafka_producer::{KafkaProducer, COMMUTE_TOPIC, MIGRATION_TOPIC};
use crate::kafka::{ticks_consumer, travel_consumer};
use crate::run_mode::RunMode;
use crate::state_machine::DiseaseHandler;
use crate::transport::engine_handlers::{EngineHandlers, KafkaImplEngineHandler};
use crate::utils::environment;
use crate::KafkaTransport;

pub struct KafkaConsumer<'a> {
    engine_id: &'a str,
    consumer: StreamConsumer,
}

impl KafkaConsumer<'_> {
    pub fn new<'a>(engine_id: &'a str, topics: &[&str]) -> KafkaConsumer<'a> {
        let kafka_url = environment::kafka_url();
        let consumer: StreamConsumer = ClientConfig::new()
            .set("bootstrap.servers", kafka_url.as_str())
            .set("group.id", &*format!("request_{}", engine_id))
            .set("auto.offset.reset", "earliest")
            .set("max.poll.interval.ms", "86400000") //max allowed
            .set("message.max.bytes", "104857600") //in order to allow message greater than 1MB
            .create()
            .expect("Consumer creation failed");

        consumer.subscribe(topics).expect("Can't subscribe to specified topics");

        KafkaConsumer { engine_id, consumer }
    }

    //Todo: Fix this function, it is unnecessary. We can directly take the config from the app rather than kafka
    pub async fn listen_loop<D: DiseaseHandler + Sync + Clone>(
        &self,
        engine_id: &str,
        run_mode: &RunMode,
        disease_handler: Option<D>,
        threads: u32,
        output_dir_path: &Path,
    ) {
        let mut message_stream: MessageStream = self.consumer.stream();
        debug!("Started the stream. Waiting for simulation request");
        while let Some(message) = message_stream.next().await {
            let simulation_config = self.parse_message(message);
            match simulation_config {
                Err(e) => {
                    error!(
                        "Received a message, but could not parse it.\n\
                        Error Details: {}",
                        e
                    );
                }
                Ok(request) => {
                    let migrators_consumer =
                        travel_consumer::start(engine_id, &[&*format!("{MIGRATION_TOPIC}{engine_id}")], "migrate");
                    let migration_stream = migrators_consumer.stream();

                    let commute_consumer =
                        travel_consumer::start(engine_id, &[&*format!("{COMMUTE_TOPIC}{engine_id}")], "commute");
                    let commute_stream = commute_consumer.stream();

                    let ticks_consumer = ticks_consumer::start(engine_id);
                    let ticks_stream = ticks_consumer.stream();
                    let producer = KafkaProducer::new();
                    let transport =
                        KafkaTransport::new(engine_id.to_string(), producer, ticks_stream, commute_stream, migration_stream);

                    //Todo: fix it there is already a kafkaProducer in the scope, try to use that (think of merging the engine handlers and transport)
                    let engine_handlers = KafkaImplEngineHandler::new(KafkaProducer::new());

                    self.run_sim(
                        request,
                        run_mode,
                        disease_handler.clone(),
                        Some(transport),
                        engine_handlers,
                        threads,
                        output_dir_path,
                    )
                    .await;
                    if let RunMode::MultiEngine { .. } = run_mode {
                        return;
                    }
                }
            };
        }
    }

    async fn run_sim<'a, D: DiseaseHandler + Sync, EH: EngineHandlers>(
        &self,
        request: Request,
        run_mode: &RunMode,
        disease_handler: Option<D>,
        transport: Option<KafkaTransport<'a>>,
        engine_handlers: EH,
        threads: u32,
        output_dir_path: &Path,
    ) {
        match request {
            Request::SimulationRequest(req) => {
                if disease_handler.is_none() {
                    let disease = req.config.get_disease();
                    let mut epidemiology = Epidemiology::new(
                        req.sim_id,
                        req.config,
                        None,
                        run_mode,
                        disease,
                        transport,
                        engine_handlers,
                        output_dir_path,
                    );
                    epidemiology.run(threads).await;
                } else {
                    let mut epidemiology = Epidemiology::new(
                        req.sim_id,
                        req.config,
                        None,
                        run_mode,
                        disease_handler.unwrap(),
                        transport,
                        engine_handlers,
                        output_dir_path,
                    );
                    epidemiology.run(threads).await;
                };
            }
            Request::MultiSimRequest(req) => {
                let travel_plan_config = Some(req.travel_plan);
                let sim_req = req.engine_configs.iter().find(|c| c.engine_id == self.engine_id);
                match sim_req {
                    None => {
                        error!("Couldn't find any work for engine_id: {}", self.engine_id)
                    }
                    Some(req) => {
                        let config = req.config.config.clone();
                        if disease_handler.is_none() {
                            let disease = config.get_disease();
                            let mut epidemiology = Epidemiology::new(
                                req.engine_id.to_string(),
                                config,
                                travel_plan_config,
                                run_mode,
                                disease,
                                transport,
                                engine_handlers,
                                output_dir_path,
                            );
                            epidemiology.run(threads).await;
                        } else {
                            let mut epidemiology = Epidemiology::new(
                                req.engine_id.to_string(),
                                config,
                                travel_plan_config,
                                run_mode,
                                disease_handler.unwrap(),
                                transport,
                                engine_handlers,
                                output_dir_path,
                            );
                            let tracer = global::tracer("epirust-trace");
                            let span = tracer.start("run");
                            let cx = Context::current_with_span(span);
                            epidemiology.run(threads).with_context(cx).await;
                        }
                    }
                }
            }
        }
    }

    fn parse_message(&self, message: Result<BorrowedMessage, KafkaError>) -> Result<Request, Box<dyn Error>> {
        let borrowed_message = message?;
        let parsed_message = borrowed_message.payload_view::<str>().unwrap()?;
        trace!("Received: {}", parsed_message);
        serde_json::from_str(parsed_message).map_err(|e| e.into())
    }
}
