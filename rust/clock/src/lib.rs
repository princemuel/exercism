use core::fmt;

#[derive(Debug, PartialEq, Eq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Self { hours, minutes }
    }

    pub fn add_minutes(&mut self, minutes: i32) -> Self {
        let total_mins = (self.hours * 60) + self.minutes;

        let total_mins = total_mins + minutes;

        let mut hours = total_mins / 60;
        let minutes = total_mins % 60;

        if hours >= 24 {
            hours -= 24
        }

        if hours < 0 {
            hours += 24
        }

        Self { hours, minutes }
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.hours, self.minutes)
    }
}
