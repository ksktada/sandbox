use crate::domain::{Product, Products};
use crate::dto::ProductDto;
use std::fs::File;
use std::io::BufReader;

fn parse_catalog() -> Products {
    let file = File::open("products.json").unwrap();
    let reader = BufReader::new(file);
    serde_json::from_reader(reader).unwrap()
}

pub fn list_products() -> Vec<ProductDto> {
    parse_catalog()
        .products
        .into_iter()
        .map(|e| e.into())
        .collect::<Vec<ProductDto>>()
}

pub fn get_product(product_id: &str) -> Option<ProductDto> {
    parse_catalog()
        .products
        .into_iter()
        .find(|e| e.id == product_id)
        .map(|p| p.into())
}

pub fn search_products(query: &str) -> Option<Vec<ProductDto>> {
    let results = parse_catalog()
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
