use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}

const DAY_HOURS: i32 = 24; // hours in one day
const HOUR_MINUTES: i32 = 60; // minutes in one hour

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Clock {
            hours: hours,
            minutes: minutes,
        }.adjust_rollover()
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock {
            hours: self.hours,
            minutes: self.minutes + minutes,
        }.adjust_rollover()
    }

    fn adjust_rollover(&self) -> Self {
        let total_minutes = self.hours * HOUR_MINUTES + self.minutes;
        Clock {
            hours: (total_minutes.div_euclid(HOUR_MINUTES)).rem_euclid(DAY_HOURS),
            minutes: total_minutes.rem_euclid(HOUR_MINUTES),
        }
    }
}
