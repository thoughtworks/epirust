extern crate csv;

use std::error::Error;
use csv::Writer;

use serde::Serialize;

#[derive(Serialize)]
pub struct Row {
    hour: i32,
    susceptible: i32,
    infected: i32
}

impl Row{
    pub fn new(hour: i32, susceptible: i32, infected: i32) -> Row{
        Row{hour, susceptible, infected}
    }
}

pub fn write(file_path: &str, data: &Vec<Row>) -> Result<(), Box<Error>> {
    let mut wtr = Writer::from_path(file_path)?;

    for row in data{
        wtr.serialize(row);
    }

    wtr.flush()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn write() {
        let a = vec![Row{hour: 1, susceptible: 10, infected: 20}, Row{hour: 2, susceptible: 20, infected: 30}];
        super::write("test.csv", &a);
    }
}