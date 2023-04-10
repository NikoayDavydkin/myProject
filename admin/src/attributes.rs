use chrono::{DateTime, Utc};
use hashbrown::HashMap;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::attribute_value;
use crate::attribute_value::AttributeValue;
use crate::attribute_value::CpuGroup;
use crate::attribute_value::ScreenResolution;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AttributeEntry {
    pub key: Uuid,
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Attributes {
    pub attributes: HashMap<Uuid, AttributeValue>,
}

#[allow(dead_code)]
impl Attributes {
    pub fn new() -> Self {
        Self {
            attributes: HashMap::new(),
        }
    }

    pub fn get(&self, id: &Uuid) -> Option<&AttributeValue> {
        self.attributes.get(id)
    }

    pub fn get_or_default(
        &self,
        id: &Uuid,
        default: &Option<AttributeValue>,
    ) -> Option<AttributeValue> {
        match self.attributes.get(id) {
            Some(value) => Some(value.clone()),
            _ => default.clone(),
        }
    }

    pub fn insert(&mut self, id: Uuid, value: AttributeValue) {
        self.attributes.insert(id, value);
    }

    pub fn get_str(&self, id: &Uuid) -> Option<&str> {
        match self.attributes.get(id) {
            Some(AttributeValue::String(value)) => Some(value),
            _ => None,
        }
    }

    pub fn get_string_or_default(
        &self,
        id: &Uuid,
        default: &Option<AttributeValue>,
    ) -> Option<String> {
        match self.attributes.get(id) {
            Some(value) => match value {
                AttributeValue::String(value) => Some(value.clone()),
                _ => None,
            },
            _ => match default {
                Some(AttributeValue::String(value)) => Some(value.clone()),
                _ => None,
            },
        }
    }

    pub fn get_datetime(&self, id: &Uuid) -> Option<&DateTime<Utc>> {
        match self.attributes.get(id) {
            Some(AttributeValue::DateTime(value)) => Some(value),
            _ => None,
        }
    }

    pub fn get_body(&self, id: &Uuid) -> Option<&Vec<attribute_value::Section>> {
        match self.attributes.get(id) {
            Some(AttributeValue::Body(value)) => Some(value),
            _ => None,
        }
    }

    pub fn get_u64(&self, id: &Uuid) -> Option<u64> {
        match self.attributes.get(id) {
            Some(AttributeValue::U64(value)) => Some(*value),
            _ => None,
        }
    }

    pub fn get_u64_or_default(&self, id: &Uuid, default: &Option<AttributeValue>) -> Option<u64> {
        match self.attributes.get(id) {
            Some(value) => match value {
                AttributeValue::U64(value) => Some(*value),
                _ => None,
            },
            _ => match default {
                Some(AttributeValue::U64(value)) => Some(*value),
                _ => None,
            },
        }
    }

    pub fn get_f64(&self, id: &Uuid) -> Option<f64> {
        match self.attributes.get(id) {
            Some(AttributeValue::F64(value)) => Some(*value),
            _ => None,
        }
    }

    pub fn get_f64_or_default(&self, id: &Uuid, default: &Option<AttributeValue>) -> Option<f64> {
        match self.attributes.get(id) {
            Some(value) => match value {
                AttributeValue::F64(value) => Some(*value),
                _ => None,
            },
            _ => match default {
                Some(AttributeValue::F64(value)) => Some(*value),
                _ => None,
            },
        }
    }

    pub fn get_sources(&self, id: &Uuid) -> Option<&Vec<attribute_value::Source>> {
        match self.attributes.get(id) {
            Some(AttributeValue::Sources(value)) => Some(value),
            _ => None,
        }
    }

    pub fn get_uuid(&self, id: &Uuid) -> Option<Uuid> {
        match self.attributes.get(id) {
            Some(AttributeValue::Uuid(value)) => Some(*value),
            _ => None,
        }
    }

    pub fn get_string_array(&self, id: &Uuid) -> Option<&Vec<String>> {
        match self.attributes.get(id) {
            Some(AttributeValue::StringArray(value)) => Some(value),
            _ => None,
        }
    }

    pub fn get_screen_resolution(&self, id: &Uuid) -> Option<&ScreenResolution> {
        match self.attributes.get(id) {
            Some(AttributeValue::ScreenResolution(value)) => Some(value),
            _ => None,
        }
    }

    pub fn get_screen_resolution_or_default(
        &self,
        id: &Uuid,
        default: &Option<AttributeValue>,
    ) -> Option<ScreenResolution> {
        match self.attributes.get(id) {
            Some(value) => match value {
                AttributeValue::ScreenResolution(value) => Some(value.clone()),
                _ => None,
            },
            _ => match default {
                Some(AttributeValue::ScreenResolution(value)) => Some(value.clone()),
                _ => None,
            },
        }
    }

    pub fn get_string_value_map(&self, id: &Uuid) -> Option<&HashMap<String, AttributeValue>> {
        match self.attributes.get(id) {
            Some(AttributeValue::StringValueMap(value)) => Some(value),
            _ => None,
        }
    }

    pub fn get_uuid_f64_map(&self, id: &Uuid) -> Option<&HashMap<Uuid, f64>> {
        match self.attributes.get(id) {
            Some(AttributeValue::UuidF64Map(value)) => Some(value),
            _ => None,
        }
    }

    pub fn get_bool(&self, id: &Uuid) -> Option<Uuid> {
        if let AttributeValue::Uuid(value) = self.attributes.get(id)? {
            Some(*value)
        } else {
            None
        }
    }

    pub fn get_cpu_group(&self, id: &Uuid) -> Option<CpuGroup> {
        if let AttributeValue::CpuGroup(value) = self.attributes.get(id)? {
            Some(value.clone())
        } else {
            None
        }
    }
}

impl From<Option<Vec<AttributeEntry>>> for Attributes {
    fn from(attributes: Option<Vec<AttributeEntry>>) -> Self {
        let attributes: HashMap<Uuid, AttributeValue> = match attributes {
            Some(attributes) => attributes
                .into_iter()
                .filter_map(|entry| match serde_json::from_str(&entry.value) {
                    Ok(value) => Some((entry.key, value)),
                    Err(error) => {
                        println!("Could not parse attribute: {}: {}", entry.value, error);
                        None
                    }
                })
                .collect(),
            _ => HashMap::new(),
        };
        Attributes { attributes }
    }
}
