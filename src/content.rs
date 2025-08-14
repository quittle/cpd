use schemars::JsonSchema;
use serde::Serialize;

use crate::{CardId, ObjectId};

#[derive(Serialize, JsonSchema, Debug)]
#[serde(deny_unknown_fields)]
pub enum Content {
    Card(CardId),
    Object(ObjectId),
}
