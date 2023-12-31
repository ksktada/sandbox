use domain::model::{Money, Product, Products};

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
        Self {
            id: value.id,
            name: value.name,
            categories: value.categories,
            description: value.description,
            picture: value.picture,
            price_usd: value.price_usd.map(|money| money.into()),
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
