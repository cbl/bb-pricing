use crate::{request::TourRequest, pricing::{PriceComposition, Amount, Price, PriceType}, rule::Modifier};

use serde::Deserialize;

#[derive(Deserialize)]
pub struct FixAmountModifier {
    amount: Amount
}

impl Modifier for FixAmountModifier {
    fn apply(&self, _: &TourRequest, pricing: &PriceComposition) -> Vec<Price> {
        vec![
            Price {
                amount: self.amount - pricing.get_amount(),
                typ: PriceType::Fix,
                meta: format!("Fix price")
            }
        ]
    }
}
