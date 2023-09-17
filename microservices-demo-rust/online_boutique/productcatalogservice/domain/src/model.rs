use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct Products {
    pub products: Vec<Product>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct Product {
    pub id: String,
    pub name: String,
    pub categories: Vec<String>,
    pub description: String,
    pub picture: String,
    pub price_usd: Option<Money>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct Money {
    pub currency_code: String,
    pub units: i64,
    pub nanos: i32,
}
