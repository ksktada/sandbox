pub mod dto;

use dto::ProductDto;
use domain::{Product, Products};
use std::fs::File;
use std::io::BufReader;

fn parse_catalog() -> Products {
    let file_name = "products.json";
    let file = File::open(file_name).unwrap();
    let reader = BufReader::new(file);
    let products: Products = serde_json::from_reader(reader).unwrap();
    products
}

pub fn list_products() -> Vec<ProductDto> {
    let products = parse_catalog();
    products
        .products
        .into_iter()
        .map(|e| e.into())
        .collect::<Vec<ProductDto>>()
}

pub fn get_product(product_id: &String) -> Option<ProductDto> {
    let products = parse_catalog();
    let product = products.products.into_iter().find(|e| &e.id == product_id);
    match product {
        Some(p) => Some(p.into()),
        None => None,
    }
}

pub fn search_products(query: &String) -> Option<Vec<ProductDto>> {
    let products = parse_catalog();
    let results = products
        .products
        .into_iter()
        .filter(|e| {
            e.name.to_lowercase().contains(&query.to_lowercase())
                || e.description.to_lowercase().contains(&query.to_lowercase())
        })
        .collect::<Vec<Product>>();
    if results.is_empty() {
        None
    } else {
        Some(
            results
                .into_iter()
                .map(|e| e.into())
                .collect::<Vec<ProductDto>>(),
        )
    }
}
