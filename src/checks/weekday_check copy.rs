use crate::{request::TourRequest, pricing::{PriceComposition, Amount, Price, PriceType}, rule::{Modifier, RuleCheck}};

use chrono::{Datelike, Weekday};
use serde::Deserialize;

#[derive(Deserialize)]
#[serde(tag = "type", content = "value")]
pub enum WeekdayCheck {
    Single(Weekday),
    Multiple(Vec<Weekday>)
}

impl RuleCheck for WeekdayCheck {
    fn applies(&self, request: &TourRequest) -> bool {
        request.sections.iter().any(|section| {
            match self {
                Self::Single(weekday) => 
                    *weekday == section.starts_at.weekday(),
                Self::Multiple(weekdays) => 
                    weekdays.contains(&section.starts_at.weekday())
            }
        })
    }
}
