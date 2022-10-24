use crate::{request::TourRequest, rule::RuleCheck};

use chrono::{Datelike, Weekday};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct WeekdaysCheck {
    pub weekdays: Vec<Weekday>,
}

impl RuleCheck for WeekdaysCheck {
    fn applies(&self, request: &TourRequest) -> bool {
        request.sections.iter().any(|section| {
            self.weekdays.contains(&section.starts_at.weekday())
        })
    }
}

#[cfg(test)]
mod tests {
    use chrono::{NaiveDate, NaiveTime};
    use crate::request::{Section, Location};
    use super::*;

    #[test]
    fn test_weekdays_check() {
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

        let check_monday_tuesday = WeekdaysCheck { weekdays: vec![
            Weekday::Mon, Weekday::Tue
        ] };
        let check_monday_friday = WeekdaysCheck { weekdays: vec![
            Weekday::Mon, Weekday::Fri
        ] };

        assert!(!check_monday_tuesday.applies(&request));
        assert!(check_monday_friday.applies(&request));
    }
}