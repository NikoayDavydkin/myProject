use chrono::{DateTime, Utc};
use hashbrown::HashMap;
use hashbrown::HashSet;
use serde_derive::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Crop {
    pub top: f64,
    pub bottom: f64,
    pub left: f64,
    pub right: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Media {
    pub url: String,
    pub crop: Option<Crop>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RichParagraph {
    pub title: String,
    pub media: Option<Media>,
    pub rich_text: Option<String>,
    pub specs: Vec<String>,
    pub pros: Vec<String>,
    pub cons: Vec<String>,
    pub asin: Option<String>,
    pub link: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Section {
    RichText(String),
    Video(String),
    RichParagraph(Box<RichParagraph>),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Source {
    pub author: Option<String>,
    pub date: Option<String>,
    pub title: Option<String>,
    pub link: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IntelAttribute {
    pub disclaimer: Option<String>,
    pub label: String,
    pub raw_value: AttributeValue,
    pub tool_tip: Option<String>,
    pub value: String,
}

#[derive(Serialize, Deserialize, Eq, PartialEq, Hash, Debug, Clone)]
pub struct IntelCoreIx {
    pub ix: u64,
    pub gen: u64,
}

#[derive(Serialize, Deserialize, Eq, PartialEq, Hash, Debug, Clone)]
pub enum CpuGroup {
    IntelCoreIx(IntelCoreIx),
    Other(String),
}

#[derive(Serialize, Deserialize, Eq, PartialEq, Hash, Debug, Clone)]
pub struct ScreenResolution {
    pub width: u64,
    pub height: u64,
}

#[allow(dead_code)]
impl ScreenResolution {
    pub fn new(width: u64, height: u64) -> Self {
        Self { width, height }
    }
}

impl fmt::Display for ScreenResolution {
    #[allow(dead_code)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}×{}", self.width, self.height)
    }
}

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum AttributeValue {
    U64(u64),
    F64(f64),
    String(String),
    Uuid(Uuid),
    Array(Vec<AttributeValue>),
    StringArray(Vec<String>),
    DateTime(DateTime<Utc>),
    Body(Vec<Section>),
    Sources(Vec<Source>),
    UuidSet(HashSet<Uuid>),
    ScreenResolution(ScreenResolution),
    StringValueMap(HashMap<String, AttributeValue>),
    UuidF64Map(HashMap<Uuid, f64>),
    IntelAttributes(HashMap<String, IntelAttribute>),
    Bool(bool),
    CpuGroup(CpuGroup),
}

#[allow(dead_code)]
impl AttributeValue {
    pub fn from_json(value: &Value) -> AttributeValue {
        match value {
            Value::String(value) => AttributeValue::String(value.to_string()),
            Value::Number(value) => AttributeValue::F64(value.as_f64().unwrap()),
            Value::Array(array) => {
                let mut are_strings = true;
                for value in array {
                    match value {
                        Value::String(_) => {}
                        _ => are_strings = false,
                    }
                }
                if are_strings {
                    let mut strings = Vec::new();
                    for value in array {
                        if let Value::String(value) = value {
                            strings.push(value.clone())
                        }
                    }
                    AttributeValue::StringArray(strings)
                } else {
                    let values = array
                        .iter()
                        .map(AttributeValue::from_json)
                        .collect::<Vec<_>>();
                    AttributeValue::Array(values)
                }
            }
            _ => {
                println!("Error: from_json({:?}) failed.", value);
                AttributeValue::String("".to_string())
            }
        }
    }

    pub fn as_str(&self) -> Option<&str> {
        match *self {
            AttributeValue::String(ref value) => Some(value),
            _ => None,
        }
    }

    pub fn as_f64(&self) -> Option<f64> {
        match *self {
            AttributeValue::F64(value) => Some(value),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_screen_resolution() {
        let value = ScreenResolution {
            width: 1920,
            height: 1080,
        };
        assert_eq!(value.to_string(), "1920×1080".to_owned());
    }
}
