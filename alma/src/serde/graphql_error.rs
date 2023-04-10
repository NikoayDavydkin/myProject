use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Error {
    pub path: Option<Vec<String>>,
    pub locations: Vec<Location>,
    pub message: String,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct Location {
    line: u64,
    column: u64,
}
