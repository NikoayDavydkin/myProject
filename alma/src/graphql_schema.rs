use async_graphql::{Context, InputObject, Object};
use bonfire_core::attribute_value::AttributeValue;
use bonfire_core::attributes::Attributes;
use hashbrown::HashMap;
use hashbrown::HashSet;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

use crate::parse_dispatcher::ParseDispatcher;

#[derive(InputObject)]
pub struct AttributeEntryInput {
    pub key: Uuid,
    pub value: String,
}

pub struct ParseResult {
    input_attributes: Attributes,
    input_category_ids: HashSet<Uuid>,
    attributes: Attributes,
    category_ids: HashSet<Uuid>,
}

#[Object]
impl ParseResult {
    async fn input_attributes(&self, _ctx: &Context<'_>) -> &Attributes {
        &self.input_attributes
    }

    async fn input_category_ids(&self, _ctx: &Context<'_>) -> Vec<Uuid> {
        self.input_category_ids.iter().cloned().collect()
    }

    async fn attributes(&self, _ctx: &Context<'_>) -> &Attributes {
        &self.attributes
    }

    async fn category_ids(&self, _ctx: &Context<'_>) -> Vec<Uuid> {
        self.category_ids.iter().cloned().collect()
    }
}

pub struct AlmaData {
    pub parser: ParseDispatcher,
}

impl AlmaData {
    #[allow(clippy::new_without_default)]
    pub async fn new() -> Self {
        Self {
            parser: ParseDispatcher::new(),
        }
    }
}

pub type Storage = Arc<RwLock<AlmaData>>;

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn parse(
        &self,
        ctx: &Context<'_>,
        attributes: Vec<AttributeEntryInput>,
        category_ids: Option<Vec<Uuid>>,
    ) -> ParseResult {
        let data = ctx.data_unchecked::<Storage>().read().await;
        let attributes: HashMap<Uuid, AttributeValue> = attributes
            .into_iter()
            .filter_map(|entry| match serde_json::from_str(&entry.value) {
                Ok(value) => Some((entry.key, value)),
                Err(error) => {
                    println!("Could not parse attribute: {}: {}", entry.value, error);
                    None
                }
            })
            .collect();
        let input_category_ids = category_ids.unwrap_or_default();
        let input_attributes = Attributes { attributes };
        let (category_ids, attributes) = data.parser.parse(
            &input_category_ids.iter().cloned().collect(),
            &input_attributes,
        );

        ParseResult {
            input_attributes,
            input_category_ids: input_category_ids.iter().cloned().collect(),
            attributes,
            category_ids,
        }
    }
}

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn no_op(&self, _ctx: &Context<'_>) -> String {
        "no op".to_string()
    }
}
