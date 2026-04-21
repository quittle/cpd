use schemars::JsonSchema;
use serde::Serialize;

use crate::{DeclareWrappedType, battle_file};

DeclareWrappedType!(ObjectId, id, battle_file::ObjectId);
DeclareWrappedType!(ObjectInstanceId, id, usize);

#[derive(PartialEq, Copy, Clone, Eq, Hash, Debug, Serialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ObjectInstance {
    pub object_id: ObjectId,
    pub object_instance_id: ObjectInstanceId,
}

impl ObjectInstance {
    pub fn new(object_id: ObjectId, object_instance_id: ObjectInstanceId) -> Self {
        Self {
            object_id,
            object_instance_id,
        }
    }
}

#[derive(Debug, Clone, Serialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Object {
    pub id: ObjectId,
    pub name: String,
    pub description: String,
    pub image: Option<String>,
}
