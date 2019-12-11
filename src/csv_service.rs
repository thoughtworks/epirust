extern crate csv;

use std::error::Error;
use csv::Writer;

#[derive(Serialize, Copy, Clone)]
pub struct Row {
    hour: i32,
    susceptible: i32,
    infected: i32,
    quarantined: i32,
    recovered: i32,
    deceased: i32
}

impl Row{

    pub fn create(hour: i32, susceptible: i32, infected: i32, quarantined:i32, recovered: i32, deceased: i32) -> Row{
        Row{hour, susceptible, infected, quarantined, recovered, deceased}
    }

    pub fn update_susceptible(&mut self, count: i32){
        self.susceptible = self.susceptible + count;
    }

    pub fn update_infected(&mut self, count:i32){
        self.infected = self.infected + count;
    }

    pub fn update_recovered(&mut self, count:i32){
        self.recovered = self.recovered + count;
    }

    pub fn update_deceased(&mut self, count:i32){
        self.deceased = self.deceased + count;
    }

    pub fn update_quarantined(&mut self, count:i32){
        self.quarantined = self.quarantined + count;
    }

    pub fn increment_hour(&mut self){
        self.hour += 1;
    }
}

pub fn write(file_path: &str, data: &Vec<Row>) -> Result<(), Box<dyn Error>> {
    let mut wtr = Writer::from_path(file_path)?;

    for row in data{
        let result = wtr.serialize(row);
        match result{
            Ok(_) => {},
            Err(e) => {
                println!("Error occurred while serializing! {:?}", e);
            }
        }
    }

    wtr.flush()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn write() {
        let a = vec![Row::create( 1, 10, 20, 0, 0, 0), Row::create(2, 20, 30, 0, 0, 0)];
        super::write("test.csv", &a);
    }
}