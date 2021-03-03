#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

const DAY_HOURS: i32 = 24; // hours in one day
const HOUR_MINUTES: i32 = 60; // minutes in one hour
const DAY_MINUTES: i32 = DAY_HOURS * HOUR_MINUTES; // minutes in one day

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Clock {
            hours: hours,
            minutes: minutes,
        }.adjust_rollover()
    }

    pub fn to_string(&self) -> String {
        format!("{:02}:{:02}", self.hours, self.minutes)
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock {
            hours: self.hours,
            minutes: self.minutes + minutes,
        }.adjust_rollover()
    }

    fn adjust_rollover(&self) -> Self {
        let mut total_minutes = self.hours * HOUR_MINUTES + self.minutes;
        while total_minutes < 0 {
            total_minutes += DAY_MINUTES;
        }
        Clock {
            hours: (total_minutes / HOUR_MINUTES) % DAY_HOURS,
            minutes: total_minutes % HOUR_MINUTES,
        }
    }
}
