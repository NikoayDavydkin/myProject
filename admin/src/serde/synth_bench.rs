use serde_derive::Deserialize;
use uuid::Uuid;

use crate::attributes::AttributeEntry;
use crate::serde::graphql_error::Error;

#[derive(Deserialize, Debug)]
pub struct SynthBenchResult {
    pub data: Option<Data>,
    pub errors: Option<Vec<Error>>,
}

#[derive(Deserialize, Debug)]
pub struct Data {
    #[serde(rename = "synthBench")]
    pub synth_bench: SynthBench,
}

#[derive(Deserialize, Debug)]
pub struct SynthBench {
    pub cpus: Vec<Cpu>,
    #[serde(rename = "synthPassmarkSingleThreadMse")]
    pub synth_passmark_single_thread_mse: f64,
}

#[derive(Deserialize, Debug)]
pub struct Cpu {
    pub id: Uuid,
    pub title: String,
    #[serde(rename = "passmarkSingleThread")]
    pub passmark_single_thread: f64,
    #[serde(rename = "synthPassmarkSingleThread")]
    pub synth_passmark_single_thread: f64,
    pub attributes: Attributes,
}

#[derive(Deserialize, Debug)]
pub struct Attributes {
    pub attributes: Vec<AttributeEntry>,
}
