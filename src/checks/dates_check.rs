use crate::{request::TourRequest, rule::RuleCheck};

use chrono::{NaiveDate};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct DatesCheck {
    pub dates: Vec<NaiveDate>,
}

impl RuleCheck for DatesCheck {
    fn applies(&self, request: &TourRequest) -> bool {
        request.sections.iter().any(|section| {
            self.dates.iter().any(|&date| {
                section.starts_at.date() == date
            })
        })
    }
}

#[cfg(test)]
mod tests {
    use chrono::{NaiveDate, NaiveTime};
    use crate::request::{Section, Location};
    use super::*;

    #[test]
    fn test_dates_check() {
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

        let other_date = DatesCheck { 
            dates: vec![NaiveDate::from_ymd(2022, 9, 17)]
        };
        let same_date = DatesCheck { 
            dates: vec![NaiveDate::from_ymd(2022, 9, 16)]
        };

        assert!(!other_date.applies(&request));
        assert!(same_date.applies(&request));
    }
}
