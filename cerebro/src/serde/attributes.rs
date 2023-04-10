use serde_derive::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Debug)]
pub struct Attributes {
    pub attributes: Vec<AttributeEntry>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AttributeEntry {
    pub key: Uuid,
    pub value: String,
}

