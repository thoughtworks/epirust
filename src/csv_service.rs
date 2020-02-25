extern crate csv;

use std::error::Error;
use csv::Writer;
use crate::events::Counts;

pub fn write(file_path: &str, data: &[Counts]) -> Result<(), Box<dyn Error>> {
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
