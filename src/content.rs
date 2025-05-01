use schemars::JsonSchema;
use serde::Serialize;

use crate::{CardId, ObjectId};

#[derive(Serialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub enum Content {
    Card(CardId),
    Object(ObjectId),
}
