mod weekdays_check;
mod clock_time_check;
mod dates_check;

use crate::{rule::{Modifier, RuleCheck}, request::TourRequest};

use serde::Deserialize;

#[derive(Deserialize)]
#[serde(tag = "type", content = "value")]
pub enum RuleCheckType {
    Weekday(weekdays_check::WeekdaysCheck),
    ClockTime(clock_time_check::ClockTimeCheck),
    DatesCheck(dates_check::DatesCheck),
}

impl RuleCheckType {
    pub fn get_check(&self) -> Box<&dyn RuleCheck> {
        match self {
            Self::Weekday(c) => Box::new(c),
            Self::ClockTime(c) => Box::new(c),
            Self::DatesCheck(c) => Box::new(c),
        }
    }
}