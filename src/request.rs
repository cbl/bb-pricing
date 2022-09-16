use chrono::{Duration, NaiveDateTime};
use serde::{Deserialize};

#[derive(Deserialize)]
pub struct Location {
    pub name: String
}

#[derive(Deserialize)]
pub struct Section {
    pub id: usize,
    pub from: Location,
    pub to: Location,
    pub distance: i64,
    pub starts_at: NaiveDateTime,
    pub ends_at: NaiveDateTime
}

impl Section {
    pub fn get_duration(&self) -> Duration {
        self.ends_at - self.starts_at
    }
}


#[derive(Deserialize)]
pub struct TourRequest {
    pub sections: Vec<Section>,
}

impl TourRequest {
    pub fn new() -> TourRequest {
        TourRequest { sections: Vec::new() }
    }
}

impl From<Vec<Section>> for TourRequest {
    fn from(sections: Vec<Section>) -> TourRequest {
        TourRequest { sections }
    }
}