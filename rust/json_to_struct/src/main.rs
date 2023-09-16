use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all(deserialize = "camelCase"))]
struct Products {
    products: Vec<Product>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all(deserialize = "camelCase"))]
struct Product {
    id: String, 
    name: String,
    categories: Vec<String>,
    description: String,
    picture: String,
    price_usd: Money,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all(deserialize = "camelCase"))]
struct Money {
    currency_code: String,
    units: u128,
    nanos: u128,
}

fn main() {
    let file_name = "products.json";
    let file = File::open(file_name).unwrap();
    let reader = BufReader::new(file);

    let deserialized: Products = serde_json::from_reader(reader).unwrap();

    println!("{:?}", deserialized);
}
