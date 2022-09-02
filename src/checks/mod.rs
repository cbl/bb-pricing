mod weekday_check;

use crate::{rule::{Modifier, RuleCheck}, request::TourRequest};

use serde::Deserialize;

#[derive(Deserialize)]
#[serde(tag = "type", content = "value")]
pub enum RuleCheckType {
    Weekday(weekday_check::WeekdayCheck)
}

impl RuleCheckType {
    pub fn get_check(&self) -> Box<&dyn RuleCheck> {
        match self {
            Self::Weekday(c) => Box::new(c),
        }
    }
}