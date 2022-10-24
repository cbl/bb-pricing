use crate::{request::TourRequest, rule::RuleCheck};

use chrono::{NaiveTime};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct ClockTimeCheck {
    pub start: NaiveTime,
    pub end: NaiveTime
}

impl RuleCheck for ClockTimeCheck {
    fn applies(&self, request: &TourRequest) -> bool {
        request.sections.iter().any(|section| {
            section.starts_at.time() > self.start &&
            section.ends_at.time() < self.end
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
                starts_at: NaiveDate::from_ymd(2022, 9, 16)
                    .and_time(NaiveTime::from_hms(13,0,0)),
                ends_at: NaiveDate::from_ymd(2022, 9, 16)
                    .and_time(NaiveTime::from_hms(14,0,0))
            }
        ]);

        let ten_to_12 = ClockTimeCheck { 
            start: NaiveTime::from_hms(10, 0, 0),
            end: NaiveTime::from_hms(12, 0, 0)
        };
        let ten_to_16 = ClockTimeCheck { 
            start: NaiveTime::from_hms(10, 0, 0),
            end: NaiveTime::from_hms(16, 0, 0)
        };

        assert!(!ten_to_12.applies(&request));
        assert!(ten_to_16.applies(&request));
    }
}
