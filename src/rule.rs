use serde::Deserialize;

use crate::{pricing::{PriceComposition, Price, Amount, PriceType}, request::TourRequest};
use crate::rules::ModifierType;
use crate::checks::RuleCheckType;

pub struct Closure {
    pub c: Box<dyn Fn(&TourRequest, &PriceComposition) -> Vec<Price>>,
}

#[derive(Deserialize)]
pub struct Rule {
    pub checks: Vec<RuleCheckType>,
    pub modifiers: Vec<ModifierType>,
}

impl Rule {
    pub fn should_be_applied(&self, request: &TourRequest) -> bool {
        !self.checks
            .iter()
            .any(|check_type| !check_type.get_check().applies(request))
    }

    pub fn get_prices(&self, request: &TourRequest, pricing: &PriceComposition) -> Vec<Price> {
        let mut prices = Vec::new();

        self.modifiers.iter().for_each(|modifier_type| {
            modifier_type
                .modifier()
                .apply(request, pricing)
                .into_iter()
                .for_each(|price| prices.push(price));
        });
        
        prices
    }
}

pub trait RuleCheck {
    fn applies(&self, request: &TourRequest) -> bool;
}

pub trait Modifier {
    fn apply(&self, request: &TourRequest, pricing: &PriceComposition) -> Vec<Price>;
}