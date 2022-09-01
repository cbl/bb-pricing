use serde::{Deserialize};

#[derive(Deserialize)]
pub struct Location {
    name: String
}

#[derive(Deserialize)]
pub struct Section {
    pub id: usize,
    pub from: Location,
    pub to: Location,
    pub distance: i32,
    pub duration: i32
}


#[derive(Deserialize)]
pub struct TourRequest {
    pub sections: Vec<Section>,
}
