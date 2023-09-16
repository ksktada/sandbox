fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
    .type_attribute("Product", "#[derive(Deserialize)] #[serde(rename_all(deserialize = \"camelCase\"))]")
    .type_attribute("Money", "#[derive(Deserialize)] #[serde(rename_all(deserialize = \"camelCase\"))]")
    .compile(&["../../proto/online_boutique.proto"], &["../../proto"])?;
    Ok(())
}
