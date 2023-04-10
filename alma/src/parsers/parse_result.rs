use bonfire_core::attributes::Attributes;

pub struct ParseResult {
    pub start: usize,
    pub end: usize,
    pub attributes: Attributes,
}
