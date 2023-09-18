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
use usecase::{get_supported_currencies, convert};

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

// pb -> dtoの変換を実装
impl Into<MoneyDto> for Money {
    fn into(self) -> MoneyDto {
        MoneyDto {
            currency_code: self.currency_code,
            units: self.units,
            nanos: self.nanos,
        }
    }
}

// pbのserviceを実装
#[derive(Debug, Default)]
struct CurrencyServiceImpl {}

#[tonic::async_trait]
impl CurrencyService for CurrencyServiceImpl {
    async fn get_supported_currencies(&self, _request: Request<Empty>,) -> Result<Response<GetSupportedCurrenciesResponse>, Status> {
        let currency_codes = get_supported_currencies().await;
        Ok(Response::new(GetSupportedCurrenciesResponse { currency_codes }))
    }

    async fn convert(&self, request: Request<CurrencyConversionRequest>) -> Result<Response<Money>, Status> {
        let from = &request.get_ref().from.clone().unwrap().into();
        let to_code = &request.get_ref().to_code;
        let money = convert(from, to_code).await;
        Ok(Response::new(money.into()))
    }
}

pub fn server() -> Router {
    let service = CurrencyServiceImpl::default();
    Server::builder().add_service(CurrencyServiceServer::new(service))
}
