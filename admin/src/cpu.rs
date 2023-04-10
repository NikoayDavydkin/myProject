use uuid::Uuid;

use crate::attributes::Attributes;

#[derive(Debug, Clone)]
pub struct Cpu {
    pub id: Uuid,
    pub attributes: Attributes,
    pub title: String,
    pub passmark_single_thread: f64,
    pub synth_passmark_single_thread: f64,
}
