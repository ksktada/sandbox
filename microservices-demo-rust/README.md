# Microservices Demo by Rust

## 参考

- microservices-demo
  - https://github.com/GoogleCloudPlatform/microservices-demo
- grpc by rust (tonic)
  - isntall protoc
    - https://github.com/hyperium/tonic#windows
  - https://github.com/hyperium/tonic/blob/master/examples/helloworld-tutorial.md

## アーキテクチャ

- 1サービス1クレートとする
- Protocol Buffersは1ファイルを共有
  - 分割できるかどうかは分からず
    - できそう

## サービス起動

```sh
# at /path/to/online_boutiique
# cargo run -p <service_name>
cargo run -p productcatalogservice
```

## Cargo.toml (workspace)

- 各サービスをmermbersに指定
- resolverは2とする

```toml
[workspace]
members = [
    "adservice",
    "cartservice",
    ...
]
resolver = "2"
```

## Cargo.toml (service)

```toml
[package]
# サービス名
name = "productcatalogservice"
version = "0.1.0"
edition = "2021"
# デフォルトとするbinのnameを指定。--binで指定しないとこれが実行される
default-run = "productcatalogservice"
build="../build.rs" # gRPC用のスタブを作成するスクリプト

# サーバ
[[bin]]
name = "productcatalogservice"
path = "src/server.rs"

# クライアント
[[bin]]
name = "productcatalogservice-client"
path = "src/client.rs"

[dependencies]
tonic = "0.10.0"
prost = "0.12.0"
tokio = { version = "1.32.0", features = ["rt-multi-thread", "macros"] }

# build.rsで使用するクレーと
[build-dependencies]
tonic-build = {version="0.10.0", features = ["prost"] }
```
