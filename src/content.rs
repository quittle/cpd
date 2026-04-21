use schemars::JsonSchema;
use serde::Serialize;

use crate::{CardInstance, ObjectInstance};

#[derive(Serialize, JsonSchema, Debug)]
#[serde(deny_unknown_fields)]
pub enum Content {
    Card(CardInstance),
    Object(ObjectInstance),
}
