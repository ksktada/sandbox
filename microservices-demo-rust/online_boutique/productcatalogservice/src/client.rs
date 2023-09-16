// hello_server.proto 内のアイテムをモジュールとしてインポート
pub mod online_boutique {
    use serde::Deserialize;
    tonic::include_proto!("online_boutique");
}

// 上記モジュール内のアイテムを呼び出す
use online_boutique::product_catalog_service_client::ProductCatalogServiceClient;
use online_boutique::Empty;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = ProductCatalogServiceClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(Empty{});

    let response = client.list_products(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}