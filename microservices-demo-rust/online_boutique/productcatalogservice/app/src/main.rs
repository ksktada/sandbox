use grpc;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse().unwrap();
    let server = grpc::router();

    println!("ProductCatalogServiceServer listening on {}", addr);

    server.serve(addr).await?;

    Ok(())
}
