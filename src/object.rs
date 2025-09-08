use schemars::JsonSchema;
use serde::Serialize;

use crate::{DeclareWrappedType, battle_file};

DeclareWrappedType!(ObjectId, id, battle_file::ObjectId);

#[derive(Debug, Clone, Serialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Object {
    pub id: ObjectId,
    pub name: String,
    pub description: String,
    pub image: Option<String>,
}
