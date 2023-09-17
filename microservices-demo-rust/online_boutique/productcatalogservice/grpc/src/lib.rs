pub mod pb;

use pb::online_boutique::product_catalog_service_server::{
    ProductCatalogService, ProductCatalogServiceServer,
};
use pb::online_boutique::{
    Empty, GetProductRequest, ListProductsResponse, Product, SearchProductsRequest,
    SearchProductsResponse,
};
use tonic::transport::server::Router;
use tonic::{transport::Server, Request, Response, Status};
use usecase::{get_product, list_products, search_products};

#[derive(Debug, Default)]
struct ProductCatalogServiceImpl {}

#[tonic::async_trait]
impl ProductCatalogService for ProductCatalogServiceImpl {
    async fn list_products(
        &self,
        request: Request<Empty>,
    ) -> Result<Response<ListProductsResponse>, Status> {
        println!("Got a request: {:?}", request);
        let products = list_products().into_iter().map(|e| e.into()).collect();
        let response: ListProductsResponse = ListProductsResponse { products };
        Ok(Response::new(response))
    }

    async fn get_product(
        &self,
        request: Request<GetProductRequest>,
    ) -> Result<Response<Product>, Status> {
        println!("Got a request: {:?}", request);
        let product_id = &request.get_ref().id;
        match get_product(product_id) {
            Some(product_dto) => Ok(Response::new(Product::from(product_dto))),
            None => Err(Status::ok("not found.")),
        }
    }

    async fn search_products(
        &self,
        request: Request<SearchProductsRequest>,
    ) -> Result<Response<SearchProductsResponse>, Status> {
        println!("Got a request: {:?}", request);
        let query = &request.get_ref().query;
        match search_products(query) {
            Some(products) => {
                let results = products.into_iter().map(|e| e.into()).collect();
                Ok(Response::new(SearchProductsResponse { results }))
            }
            None => Err(Status::ok("not found.")),
        }
    }
}

pub fn server() -> Router {
    let service = ProductCatalogServiceImpl::default();
    Server::builder().add_service(ProductCatalogServiceServer::new(service))
}
