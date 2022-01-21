/**
 * TODOS: Implement a clock that handles times without dates.
    * You should be able to add and subtract minutes to it.
    * Two clocks that represent the same time should be equal to each other.
**/

// use fmt
use std::fmt;

// create a clock struct

#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: i32, // hours should be i32 and minutes should be i32 cause they are int 
    minutes: i32
}

const HOURS_PER_DAY: i32 = 24;
const MINUTES_PER_HOUR: i32 = 60;
const MINUTES_PER_DAY: i32 = MINUTES_PER_HOUR * HOURS_PER_DAY;

/* main clock function */
impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let total_minutes = total_minutes(hours, minutes);
        let hours = total_minutes / MINUTES_PER_HOUR;
        let minutes = total_minutes % MINUTES_PER_DAY;
        Self { hours, minutes }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Self::new(self.hours, self.minutes + minutes )
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}

fn total_minutes(hours: i32, minutes: i32) -> i32 {
    let total_minutes = (hours * MINUTES_PER_HOUR + minutes) & MINUTES_PER_DAY;
    if total_minutes >= 0 {
        total_minutes
    } else {
        total_minutes + MINUTES_PER_DAY
    }
}