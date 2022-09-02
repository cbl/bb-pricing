mod fix_amount;

use crate::rule::Modifier;

use serde::Deserialize;

#[derive(Deserialize)]
#[serde(tag = "type", content = "value")]
pub enum ModifierType {
    FixAmount(fix_amount::FixAmountModifier)
}

impl ModifierType {
    pub fn modifier(&self) -> Box<&dyn Modifier> {
        match self {
            Self::FixAmount(m) => Box::new(m),
        }
    }
}