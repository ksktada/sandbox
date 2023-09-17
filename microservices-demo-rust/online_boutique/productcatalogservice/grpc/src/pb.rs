// online_boutique.proto 内のアイテムをモジュールとしてインポート
pub mod online_boutique {
    tonic::include_proto!("online_boutique");
}

use online_boutique::{Money, Product};
use usecase::dto::{MoneyDto, ProductDto};

// dto -> pb 変換を実装
impl From<MoneyDto> for Money {
    fn from(value: MoneyDto) -> Self {
        Self {
            currency_code: value.currency_code,
            units: value.units,
            nanos: value.nanos,
        }
    }
}

impl From<ProductDto> for Product {
    fn from(value: ProductDto) -> Self {
        let price_usd = match value.price_usd {
            Some(money_dto) => Some(money_dto.into()),
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
