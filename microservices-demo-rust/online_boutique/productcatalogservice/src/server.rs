use tonic::{transport::Server, Request, Response, Status};

// online_boutique.proto 内のアイテムをモジュールとしてインポート
pub mod online_boutique {
    tonic::include_proto!("online_boutique"); 
}

// 上記モジュール内のアイテムを呼び出す
use online_boutique::product_catalog_service_server::{ProductCatalogService, ProductCatalogServiceServer};
use online_boutique::{Empty, Product, Money, ListProductsResponse, GetProductRequest, SearchProductsRequest, SearchProductsResponse};

#[derive(Debug, Default)]
pub struct ProductCatalogServiceImpl {}

#[tonic::async_trait]
impl ProductCatalogService for ProductCatalogServiceImpl {
    async fn list_products(
        &self,
        request: Request<Empty>, 
    ) -> Result<Response<ListProductsResponse>, Status> { 
        println!("Got a request: {:?}", request);

        let products: Vec<Product> = vec![Product {
            id: "1".to_string(), 
            name: "test".to_string(),
            categories: vec!["test".to_string()],
            description: "this is a test".to_string(),
            picture: "maybe url".to_string(),
            price_usd: Some(Money {
                currency_code: "USD".to_string(),
                units: 1,
                nanos: 3,
            }),
        }];

        let response: ListProductsResponse = ListProductsResponse {
            products,
        };

        Ok(Response::new(response)) 
    }

    async fn get_product(&self, request: Request<GetProductRequest>) -> Result<Response<Product>, Status> {
        Ok(Response::new(Product {
            id: "1".to_string(), 
            name: "test".to_string(),
            categories: vec!["test".to_string()],
            description: "this is a test".to_string(),
            picture: "maybe url".to_string(),
            price_usd: Some(Money {
                currency_code: "USD".to_string(),
                units: 1,
                nanos: 3,
            }),
        }))
    }

    async fn search_products(&self, request: Request<SearchProductsRequest>) -> Result<Response<SearchProductsResponse>, Status> {
        let results: Vec<Product> = vec![Product {
            id: "1".to_string(), 
            name: "test".to_string(),
            categories: vec!["test".to_string()],
            description: "this is a test".to_string(),
            picture: "maybe url".to_string(),
            price_usd: Some(Money {
                currency_code: "USD".to_string(),
                units: 1,
                nanos: 3,
            }),
        }];

        let response: SearchProductsResponse = SearchProductsResponse {
            results,
        };

        Ok(Response::new(response)) 
    }

}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse().unwrap();
    let server = ProductCatalogServiceImpl::default();

    println!("ProductCatalogServiceServer listening on {}", addr);

    Server::builder()
        .add_service(ProductCatalogServiceServer::new(server))
        .serve(addr)
        .await?;

    Ok(())
}