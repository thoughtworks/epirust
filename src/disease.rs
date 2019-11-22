pub mod small_pox {
    static REGULAR_TRANSMISSION_START_DAY: i32 = 10;
    static HIGH_TRANSMISSION_START_DAY: i32 = 16;
    static DISEASE_LAST_DAY: i32 = 22;
    static REGULAR_TRANSMISSION_RATE: f64 = 0.05;
    static HIGH_TRANSMISSION_RATE: f64 = 0.5;

    pub fn get_current_transmission_rate(infection_day: i32) -> f64 {
        if REGULAR_TRANSMISSION_START_DAY < infection_day && infection_day <= HIGH_TRANSMISSION_START_DAY {
            return REGULAR_TRANSMISSION_RATE;
        } else if HIGH_TRANSMISSION_START_DAY < infection_day && infection_day <= DISEASE_LAST_DAY {
            return HIGH_TRANSMISSION_RATE;
        }
        return 0.0;
    }
}

#[test]
fn get_current_transmission_rate(){
    let infection_rate = small_pox::get_current_transmission_rate(12);
    assert_eq!(infection_rate, 0.05);

    let infection_rate = small_pox::get_current_transmission_rate(22);
    assert_eq!(infection_rate, 0.5);
}