
use bb_pricing::pricing::{BasePrice, PriceComposition};
use bb_pricing::request::TourRequest;
use bb_pricing::rule::Rule;
use serde::Deserialize;

use std::io::{self, Stdin};
use std::io::prelude::*;
use std::str::FromStr;

#[derive(Deserialize)]
pub struct Input {
    pub tour_request: TourRequest,
    pub base_price: BasePrice,
    pub rules: Vec<Rule>
}

fn get_std_in() -> String {
    let stdin = io::stdin();

    let mut input = String::new();

    for line in stdin.lock().lines() {
        input += &line.unwrap();
    }

    input
}

fn main() {
    let input = serde_json::from_str::<Input>(&get_std_in()).unwrap();

    let pricing = PriceComposition::from((
        &input.tour_request,
        input.base_price,
        input.rules
    ));

    let output = serde_json::to_string_pretty(&pricing).unwrap();

    println!("{}", output);
}
