use grpc::online_boutique;
use online_boutique::product_catalog_service_client::ProductCatalogServiceClient;
use online_boutique::{Empty, GetProductRequest, SearchProductsRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = ProductCatalogServiceClient::connect("http://[::1]:50051").await?;

    // list_products
    println!("list_products");
    println!("------------------------");
    let request1 = tonic::Request::new(Empty {});
    let response1 = client.list_products(request1).await?;
    println!("RESPONSE={:?}", response1);
    println!("------------------------\n");

    // get_products
    println!("get_product");
    println!("------------------------");
    let request2 = tonic::Request::new(GetProductRequest {
        id: "OLJCESPC7Z".to_string(),
    });
    let response2 = client.get_product(request2).await?;
    println!("RESPONSE={:?}", response2);
    println!("------------------------\n");

    // search_products
    println!("search_product");
    println!("------------------------");
    let request3 = tonic::Request::new(SearchProductsRequest {
        query: "glass".to_string(),
    });
    let response3 = client.search_products(request3).await?;
    println!("RESPONSE={:?}", response3);
    println!("------------------------\n");
    Ok(())
}
