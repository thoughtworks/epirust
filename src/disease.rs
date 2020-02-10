pub mod small_pox {
    use crate::random_wrapper::RandomWrapper;
    use rand::Rng;

    static REGULAR_TRANSMISSION_START_DAY: i32 = 10;
    static HIGH_TRANSMISSION_START_DAY: i32 = 16;
    static DISEASE_LAST_DAY: i32 = 22;
    static REGULAR_TRANSMISSION_RATE: f64 = 0.05;
    static HIGH_TRANSMISSION_RATE: f64 = 0.5;
    static DEATH_RATE: f64 = 0.2;

    pub fn get_current_transmission_rate(infection_day: i32) -> f64 {
        if REGULAR_TRANSMISSION_START_DAY < infection_day && infection_day <= HIGH_TRANSMISSION_START_DAY {
            return REGULAR_TRANSMISSION_RATE;
        } else if HIGH_TRANSMISSION_START_DAY < infection_day && infection_day <= DISEASE_LAST_DAY {
            return HIGH_TRANSMISSION_RATE;
        }
        0.0
    }

    pub fn to_be_quarantined(infection_day: i32) -> bool {
        let transmission_rate = get_current_transmission_rate(infection_day);
        if transmission_rate >= HIGH_TRANSMISSION_RATE {
            return true;
        }
        false
    }

    pub fn get_disease_last_day() -> i32 {
        DISEASE_LAST_DAY
    }

    pub fn to_be_deceased(rng: &mut RandomWrapper) -> bool {
        if rng.get().gen_bool(DEATH_RATE) {
            return true;
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_current_transmission_rate() {
        let infection_rate = small_pox::get_current_transmission_rate(12);
        assert_eq!(infection_rate, 0.05);

        let infection_rate = small_pox::get_current_transmission_rate(22);
        assert_eq!(infection_rate, 0.5);
    }

    #[test]
    fn to_be_quarantined() {
        let actual = small_pox::to_be_quarantined(12);
        assert_eq!(actual, false);

        let actual = small_pox::to_be_quarantined(22);
        assert_eq!(actual, true);
    }
}
