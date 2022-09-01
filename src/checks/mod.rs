
use crate::rule::Modifier;

use serde::Deserialize;

#[derive(Deserialize)]
#[serde(tag = "type")]
pub enum RuleCheckType {

}

impl RuleCheckType {
    pub fn applies(&self) -> bool {
        true
        // match self {

        // }
    }
}