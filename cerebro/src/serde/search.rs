use hashbrown::HashSet;
use serde_derive::Deserialize;
use uuid::Uuid;

use crate::serde::attributes::Attributes;
use crate::serde::graphql_error::Error;

#[derive(Deserialize, Debug)]
pub struct SearchResult {
    pub data: Option<GrammarsData>,
    pub errors: Option<Vec<Error>>,
}

#[derive(Deserialize, Debug)]
pub struct GrammarsData {
    pub search: Search,
}

#[derive(Deserialize, Debug)]
pub struct Search {
    pub content: Content,
}

#[derive(Deserialize, Debug)]
pub struct Content {
    pub products: Vec<Product>,
}

#[derive(Deserialize, Debug)]
pub struct Product {
    pub id: Uuid,
    #[serde(rename = "categoryIds")]
    pub category_ids: HashSet<Uuid>,
    pub attributes: Attributes,
}
