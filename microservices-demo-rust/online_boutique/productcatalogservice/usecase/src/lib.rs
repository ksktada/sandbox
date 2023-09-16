use productcatalogservice_domain::{self, ProductDto, ProductsDto};
use std::fs::File;
use std::io::BufReader;

fn parse_catalog() -> ProductsDto {
    let file_name = "products.json";
    let file = File::open(file_name).unwrap();
    let reader = BufReader::new(file);
    let products_dto: ProductsDto = serde_json::from_reader(reader).unwrap();
    products_dto
}

pub fn list_products() -> Vec<ProductDto> {
    let products_dto = parse_catalog();
    products_dto.products
}

pub fn get_product(product_id: &String) -> Option<ProductDto> {
    let products_dto = parse_catalog();
    products_dto
        .products
        .into_iter()
        .find(|e| &e.id == product_id)
}

pub fn search_products(query: &String) -> Option<Vec<ProductDto>> {
    let products_dto = parse_catalog();
    let found = products_dto
        .products
        .into_iter()
        .filter(|e| {
            e.name.to_lowercase().contains(&query.to_lowercase())
                || e.description.to_lowercase().contains(&query.to_lowercase())
        })
        .collect::<Vec<ProductDto>>();
    if found.is_empty() {
        None
    } else {
        Some(found)
    }
}
