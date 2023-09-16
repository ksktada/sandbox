use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all(deserialize = "camelCase"))]
struct Products {
    products: Vec<Product>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all(deserialize = "camelCase"))]
struct Product {
    id: String, 
    name: String,
    categories: Vec<String>,
    description: String,
    picture: String,
    price_usd: Option<Money>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
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

    // iter or iter_into
    let list1 = deserialized.products.clone();
    let list2 = deserialized.products;
    let a = "OLJCESPC7Z".to_string();

    println!("----------------------");

    // `iter()` for vecs yields ref
    let found_ref = list1.iter().find(|&e| &e.id == &a);
    let found_value = list2.into_iter().find(|e| e.id == a);

    // `into_iter()` for vecs yields value
    println!("{:?}", found_ref);
    println!("{:?}", found_value);
}
