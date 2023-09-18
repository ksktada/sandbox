// online_boutique.proto 内のアイテムをモジュールとしてインポート
pub mod online_boutique {
    tonic::include_proto!("online_boutique");
}

use online_boutique::currency_service_server::{
    CurrencyService, CurrencyServiceServer,
};
use online_boutique::{
    Empty, GetSupportedCurrenciesResponse, CurrencyConversionRequest, Money
};
use tonic::transport::server::Router;
use tonic::{transport::Server, Request, Response, Status};
use usecase::dto::MoneyDto;

// dto -> pbの変換を実装
impl From<MoneyDto> for Money {
    fn from(value: MoneyDto) -> Self {
        Self {
            currency_code: value.currency_code,
            units: value.units,
            nanos: value.nanos,
        }
    }
}

// pbのserviceを実装
#[derive(Debug, Default)]
struct CurrencyServiceImpl {}

#[tonic::async_trait]
impl CurrencyService for CurrencyServiceImpl {
    async fn get_supported_currencies(&self, _request: Request<Empty>,) -> Result<Response<GetSupportedCurrenciesResponse>, Status> {
        let currency_codes = vec!["USD".to_string()];
        Ok(Response::new(GetSupportedCurrenciesResponse { currency_codes }))
    }

    async fn convert(&self, _request: Request<CurrencyConversionRequest>) -> Result<Response<Money>, Status> {
        let money = Money {
            currency_code: "USD".to_string(),
            units: 1,
            nanos: 9000
        };
        Ok(Response::new(money))
    }
}

pub fn server() -> Router {
    let service = CurrencyServiceImpl::default();
    Server::builder().add_service(CurrencyServiceServer::new(service))
}
