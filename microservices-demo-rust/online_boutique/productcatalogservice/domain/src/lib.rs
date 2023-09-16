use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct ProductsDto {
    pub products: Vec<ProductDto>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct ProductDto {
    pub id: String, 
    pub name: String,
    pub categories: Vec<String>,
    pub description: String,
    pub picture: String,
    pub price_usd: Option<MoneyDto>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct MoneyDto {
    pub currency_code: String,
    pub units: i64,
    pub nanos: i32,
}