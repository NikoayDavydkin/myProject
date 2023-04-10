use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct Search {
    pub search: Content,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct Content {
    pub content: Products,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct Products {
    pub products: Vec<Product>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug, Eq, Ord, PartialOrd)]
pub struct Product {
    pub id: Uuid,
    pub title: String,
    pub imageUrl: String,
    pub seller: String,
    pub buyUrl: String,
    pub updated: String,
    pub selectOffer: SelectOffer,
    pub details: Vec<Detail>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug, Eq, Ord, PartialOrd)]
pub struct SelectOffer {
    pub price: i32,
    pub shipping: i32,
    pub url: String,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug, Eq, Ord, PartialOrd)]
pub struct Detail {
    pub text: String,
}
