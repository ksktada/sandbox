use domain::{Money, Product, Products};
use std::fs::File;
use std::io::BufReader;

pub struct ProductsDto {
    pub products: Vec<ProductDto>,
}

pub struct ProductDto {
    pub id: String,
    pub name: String,
    pub categories: Vec<String>,
    pub description: String,
    pub picture: String,
    pub price_usd: Option<MoneyDto>,
}

pub struct MoneyDto {
    pub currency_code: String,
    pub units: i64,
    pub nanos: i32,
}

impl From<Products> for ProductsDto {
    fn from(value: Products) -> Self {
        let products = value
            .products
            .into_iter()
            .map(|e| e.into())
            .collect::<Vec<ProductDto>>();
        Self { products }
    }
}

impl From<Product> for ProductDto {
    fn from(value: Product) -> Self {
        let price_usd = match value.price_usd {
            Some(money) => Some(money.into()),
            None => None,
        };
        Self {
            id: value.id,
            name: value.name,
            categories: value.categories,
            description: value.description,
            picture: value.picture,
            price_usd,
        }
    }
}

impl From<Money> for MoneyDto {
    fn from(value: Money) -> Self {
        Self {
            currency_code: value.currency_code,
            units: value.units,
            nanos: value.nanos,
        }
    }
}

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
