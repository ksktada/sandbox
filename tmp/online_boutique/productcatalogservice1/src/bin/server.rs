use productcatalogservice::grpc;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse().unwrap();
    let server = grpc::server();

    println!("Server listening on {}", addr);

    server.serve(addr).await?;

    Ok(())
}