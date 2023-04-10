use anyhow::Result;
use bonfire_core::attributes::Attributes;

use reqwest::StatusCode;
use serde_json::json;
use std::env;
use std::fmt;
use uuid::Uuid;

use crate::graphql_schema::Product;
use crate::serde::search::SearchResult;

#[derive(Debug)]
struct BonfireError {
    message: String,
}

impl BonfireError {
    pub fn new(message: &str) -> Self {
        let message = message.to_owned();
        Self { message }
    }
}

impl std::error::Error for BonfireError {}

impl fmt::Display for BonfireError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

pub async fn search(category: Uuid) -> Result<Vec<Product>> {
    let query = "query search($category: ID) {
  search(category: $category, pageSize: 1000000) {
    content {
      __typename
      ... on Search {
        products {
          id
          categoryIds
          attributes {
             attributes {
              key
              value
            }
          }
        }
      }
    }
  }
}";
    let request_json = json!({ "query": query, "variables": {"category": category} });

    let client = reqwest::Client::new();
    let response = client
        .post(
            env::var("BONFIRE_URL")
                .map_err(|_| BonfireError::new("Environment variable BONFIRE_URL not found"))?,
        )
        .header("Content-Type", "application/json")
        .body(request_json.to_string())
        .send()
        .await?;

    match response.status() {
        StatusCode::OK => {
            let result: SearchResult = serde_json::from_str(&response.text().await?)?;
            match result.data {
                Some(data) => Ok(data
                    .search
                    .content
                    .products
                    .into_iter()
                    .map(|product| {
                        Product::new(
                            product.id,
                            product.category_ids,
                            Attributes {
                                attributes: product
                                    .attributes
                                    .attributes
                                    .into_iter()
                                    .filter_map(|entry| match serde_json::from_str(&entry.value) {
                                        Ok(value) => Some((entry.key, value)),
                                        Err(error) => {
                                            println!(
                                                "Could not parse attribute: {}: {}",
                                                entry.value, error
                                            );
                                            None
                                        }
                                    })
                                    .collect(),
                            },
                        )
                    })
                    .collect()),
                _ => {
                    return Err(BonfireError::new(&std::format!(
                        "Call to Bonfire failed: {:?}",
                        result
                            .errors
                            .ok_or_else(|| BonfireError::new("data and errors both null")),
                    ))
                    .into())
                }
            }
        }
        _ => Err(BonfireError::new(&std::format!("Search failed: {:?}", response)).into()),
    }
}
