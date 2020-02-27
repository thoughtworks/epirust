extern crate csv;

use std::error::Error;

use csv::Writer;

use crate::events::{Counts, Listener};
use crate::geography::Point;

pub struct CsvListener {
    output_file_name: String,
    counts: Vec<Counts>,
}

impl CsvListener {
    pub fn new(output_file_name: String) -> CsvListener {
        CsvListener { output_file_name, counts: Vec::new() }
    }

    pub fn write(file_path: &String, data: &Vec<Counts>) -> Result<(), Box<dyn Error>> {
        let mut wtr = Writer::from_path(file_path)?;

        for row in data {
            let result = wtr.serialize(row);
            match result {
                Ok(_) => {}
                Err(e) => {
                    println!("Error occurred while serializing! {:?}", e);
                }
            }
        }

        wtr.flush()?;
        Ok(())
    }
}

impl Listener for CsvListener {
    fn counts_updated(&mut self, counts: Counts) {
        self.counts.push(counts);
    }

    fn simulation_ended(&mut self) {
        CsvListener::write(&self.output_file_name, &self.counts)
            .expect("Failed to write to file");
    }

    fn citizen_got_infected(&mut self, _cell: &Point) {
    }
}
