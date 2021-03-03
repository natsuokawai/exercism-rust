#[derive(Debug)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl PartialEq for Clock {
    fn eq(&self, other: &Self) -> bool {
        self.hours == other.hours && self.minutes == other.minutes
    }
}

impl Eq for Clock {}

const DAY_HOURS: i32 = 24;
const HOUR_MINUTES: i32 = 60;

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
        let mut hour = self.hours + self.minutes / HOUR_MINUTES;
        while hour < 0 {
            hour += DAY_HOURS;
        }
        Clock {
            hours: hour % DAY_HOURS,
            minutes: self.minutes % HOUR_MINUTES,
        }
    }
}
