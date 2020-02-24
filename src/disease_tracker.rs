use crate::geography::Point;
use fxhash::FxHashMap;

pub struct Hotspot {
    disease_hotspot_tracker: FxHashMap<Point, i32>
}

impl Hotspot {
    pub fn new() -> Hotspot {
        let disease_hotspot_tracker = FxHashMap::default();
        Hotspot{disease_hotspot_tracker}
    }

    pub fn update(&mut self, cell: &Point){
        let counter = self.disease_hotspot_tracker.entry(*cell).or_insert(0);
        *counter += 1;
    }
}

#[cfg(test)]
mod tests{
    use crate::disease_tracker::Hotspot;
    use fxhash::FxHashMap;
    use crate::geography::Point;

    #[test]
    fn should_initialize(){
        let tracker = Hotspot::new();
        assert_eq!(tracker.disease_hotspot_tracker.len(), 0);
    }

    #[test]
    fn should_add_new_entry(){
        let mut tracker = Hotspot::new();
        let current_point = Point::new(0, 1);

        tracker.update(&current_point);

        assert_eq!(*tracker.disease_hotspot_tracker.get(&current_point).unwrap(), 1);
    }

    #[test]
    fn should_update_tracker(){
        let mut tracker = Hotspot::new();
        let current_point = Point::new(0, 1);

        tracker.update(&current_point);
        tracker.update(&current_point);

        assert_eq!(*tracker.disease_hotspot_tracker.get(&current_point).unwrap(), 2);
    }
}