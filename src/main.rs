
use bb_pricing::pricing::{BasePrice, PriceComposition};
use bb_pricing::request::TourRequest;
use bb_pricing::rule::Rule;
use serde::Deserialize;
use clap::Parser;
use serde::de::Error;

use std::io::{self};
use std::io::prelude::*;

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
        input += &format!("\n");
    }

    input
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
   /// Whether to pretty print the output.
   #[clap(short, long)]
   pretty: bool,
}

fn main() {
    let args = Args::parse();

    let input_string = get_std_in();

    let input = serde_json::from_str::<Input>(&input_string).unwrap();

    let pricing = PriceComposition::from((
        &input.tour_request,
        input.base_price,
        input.rules
    ));

    let output = (if args.pretty { 
        serde_json::to_string_pretty(&pricing)
    } else { 
        serde_json::to_string(&pricing)
    }).unwrap();

    println!("{}", output);
}
