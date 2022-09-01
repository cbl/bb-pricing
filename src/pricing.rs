use serde::{Deserialize, Serialize};

use crate::rule::Rule;
use crate::request::TourRequest;

pub type Amount = i32;

#[derive(Deserialize)]
pub struct BasePrice {
    pub base: Amount,
    pub per_distance: Amount,
    pub per_duration: Amount,
}

#[derive(Serialize)]
pub struct Price {
    pub amount: Amount,
    pub typ: PriceType,
    pub meta: String
}

impl Price {
    pub fn new(amount: Amount, typ: PriceType, meta: String) -> Price {
        Price { amount, typ, meta }
    }
}

#[derive(Serialize)]
pub enum PriceType {
    Base,
    PerDuration,
    PerDistance,
    Fix
}

#[derive(Serialize)]
pub struct PriceComposition {
    prices: Vec<Price>,
    amount: Amount
}

impl PriceComposition {
    pub fn new() -> PriceComposition {
        PriceComposition {
            prices: Vec::new(),
            amount: Amount::default()
        }
    }

    pub fn push(&mut self, price: Price) {
        self.prices.push(price);
        self.amount = self.prices.iter().map(|price| price.amount).sum();
    }

    pub fn is_fix(&self) -> bool {
        self.prices.iter().any(|price| match price.typ {
            PriceType::Fix => true,
            _ => false
        })
    }

    pub fn get_prices(&self) -> &Vec<Price> {
        &self.prices
    }

    pub fn get_amount(&self) -> Amount {
        self.amount
    }
}

impl From<(&TourRequest, BasePrice)> for PriceComposition {
    fn from((request, base_price): (&TourRequest, BasePrice)) -> Self {
        let mut pricing = PriceComposition::new();

        // base price
        pricing.push(Price::new(
            base_price.base, 
            PriceType::Base, 
            "Base Price".to_string()
        ));

        request.sections.iter().for_each(|section| {
            // price per distance
            pricing.push(Price::new(
                base_price.per_distance * section.distance, 
                PriceType::PerDistance,
                format!("Price/Distance for section {}", section.id).to_string())
            );

            // price per duration
            pricing.push(Price::new(
                base_price.per_duration * section.duration, 
                PriceType::PerDuration,
                format!("Price/Duration for section {}", section.id).to_string())
            );
        });

        pricing
    }
}

impl From<(&TourRequest, BasePrice, Vec<Rule>)> for PriceComposition {
    fn from((request, base_price, rules): (&TourRequest, BasePrice, Vec<Rule>)) -> Self {
        let mut pricing = PriceComposition::from((request, base_price));

        // apply pricing rules
        rules.iter().for_each(|rule| {
            if pricing.is_fix() {
                return;
            }

            if !rule.should_be_applied() {
                return;
            }

            rule.get_prices(&request, &pricing).into_iter().for_each(|price| {
                pricing.push(price);
            });
        });

        pricing
    }
}