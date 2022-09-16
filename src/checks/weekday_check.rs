use crate::{request::TourRequest, rule::RuleCheck};

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

#[cfg(test)]
mod tests {
    use chrono::{NaiveDate, NaiveTime};
    use crate::request::{Section, Location};
    use super::*;

    #[test]
    fn test_single_weekday_check() {
        let request = TourRequest::from(vec![
            Section {
                id: 1,
                from: Location {name: format!("Kiel")},
                to: Location {name: format!("Leipzig")},
                distance: 500,
                starts_at: NaiveDate::from_ymd(2022, 9, 16) // 16.09.2022 -> Friday
                    .and_time(NaiveTime::from_hms(0,0,0)),
                ends_at: NaiveDate::from_ymd(2022, 9, 16)
                    .and_time(NaiveTime::from_hms(0,0,0))
            }
        ]);

        let check_monday = WeekdayCheck::Single(Weekday::Mon);
        let check_friday = WeekdayCheck::Single(Weekday::Fri);

        assert!(!check_monday.applies(&request));
        assert!(check_friday.applies(&request));
    }

    #[test]
    fn test_multiple_weekdays_check() {
        let request = TourRequest::from(vec![
            Section {
                id: 1,
                from: Location {name: format!("Kiel")},
                to: Location {name: format!("Leipzig")},
                distance: 500,
                starts_at: NaiveDate::from_ymd(2022, 9, 16) // 16.09.2022 -> Friday
                    .and_time(NaiveTime::from_hms(0,0,0)),
                ends_at: NaiveDate::from_ymd(2022, 9, 16)
                    .and_time(NaiveTime::from_hms(0,0,0))
            }
        ]);

        let check_monday_tuesday = WeekdayCheck::Multiple(vec![
            Weekday::Mon, Weekday::Tue
        ]);
        let check_monday_friday = WeekdayCheck::Multiple(vec![
            Weekday::Mon, Weekday::Fri
        ]);

        assert!(!check_monday_tuesday.applies(&request));
        assert!(check_monday_friday.applies(&request));
    }
}