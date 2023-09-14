use tonic::{transport::Server, Request, Response, Status};

// online_boutique.proto 内のアイテムをモジュールとしてインポート
pub mod online_boutique {
    tonic::include_proto!("online_boutique"); 
}

// 上記モジュール内のアイテムを呼び出す
use online_boutique::product_catalog_service_server::{ProductCatalogService, ProductCatalogServiceServer};
use online_boutique::{Empty, Product, Money, ListProductsResponse};

#[derive(Debug, Default)]
pub struct ProductCatalogServiceImpl {}

#[tonic::async_trait]
impl ProductCatalogService for ProductCatalogServiceImpl {
    async fn list_products(
        &self,
        request: Request<Empty>, 
    ) -> Result<Response<Product>, Status> { 
        println!("Got a request: {:?}", request);

        let product = Product {
            id: "1", 
            name: "test",
            description: "this is a test",
            picture: "maybe url",
            price_usd: Money {
                currency_code: "USD",
                units: 1,
                nanos: 3,
            },
        };

        Ok(Response::new(product)) 
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse().unwrap();
    let server = ProductCatalogService::default();

    println!("ProductCatalogServiceServer listening on {}", addr);

    Server::builder()
        .add_service(ProductCatalogServiceServer::new(server))
        .serve(addr)
        .await?;

    Ok(())
}