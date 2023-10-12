---
marp: true
---

# cargo workspaceによるプロジェクト構成(Rust)

---

# workspaceとは

<span style="font-size:20pt">

- パッケージ(あるいはクレート)をひとまとめにして管理する機能
- 以下のような特徴がある
  - `Cargo.lock`、実行ファイルの出力先(いわゆる`target`)が共通となる
    - 複数のクレートで共通する依存ライブラリがある場合、同じコードが利用される
      - コンパイル時間とディスクスペースの節約につながる
      - ライブラリのサイズが大きかったり、複数のクレートで構成される場合に有効
  - 共通のコマンドをパッケージをまたいで実行できる
    - `cargo check --workspace`、`cargo test --workspace`、など
  - メタ情報を共有できる
    - `version`、`author`、`description`、など

</span>

---

# プロジェクト構成の検討

- 題材
  - https://github.com/GoogleCloudPlatform/microservices-demo


---

# プロジェクト構成の検討 - workspaceなし

```
online_boutique
│
├─productcatalogservice
│  ├─src
│  │  ├─bin
│  │  │  ├─client.rs
│  │  │  └─server.rs
│  │  ├─domain.rs
│  │  ├─dto.rs
│  │  ├─grpc.rs
│  │  ├─lib.rs
│  │  └─usecase.rs
│  ├─target/... <- 1.2GB!
│  └─Cargo.toml
│
...<other services>
│
└─proto
   └─online_boutique.proto
```

---

# プロジェクト構成の検討 - workspaceなし

```
[package]
name = "productcatalogservice"
version = "0.1.0"
edition = "2021"
default-run = "server"
build="build.rs"

[[bin]]
name = "server"
path = "src/bin/server.rs"

[[bin]]
name = "client"
path = "src/bin/client.rs"

[dependencies]
tokio = { version = "1.32.0", features = ["rt-multi-thread", "macros"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
tonic = "0.10.0"
prost = "0.12.0"

[build-dependencies]
tonic-build = {version="0.10.0", features = ["prost"] }
```

---

# プロジェクト構成の検討 - workspace(サービス単位)
```
online_boutique
│
├─productcatalogservice
│  ├─src
│  │  ├─bin
│  │  │  ├─client.rs
│  │  │  └─server.rs
│  │  ├─domain.rs
│  │  ├─dto.rs
│  │  ├─grpc.rs
│  │  ├─lib.rs
│  │  └─usecase.rs
│  └─Cargo.toml <- 各サービスでもCargo.tomlが必要
│
...<other services>
│
├─Cargo.toml <- workspace用のCargo.toml
├─target/... <- workspaceを置いたところに作成される
└─proto
   └─online_boutique.proto
```

---

# プロジェクト構成の検討 - workspace(サービス単位)
```

```

---

# プロジェクト構成の検討 - サービス内を分割したら

```
├─productcatalogservice
│  ├─domain
│  │  ├─src/...
│  │  └─Cargo.toml
│  ├─entry
│  │  ├─src/...
│  │  └─Cargo.toml
│  ├─grpc
│  │  ├─src/...
│  │  └─Cargo.toml
│  ├─usecase
│  │  ├─src/...
│  │  └─Cargo.toml
│  └─target
│     ├─debug/...
│     └─Cargo.toml
└─proto
   └─online_boutique.proto
```
---

# プロジェクト構成の検討 - サービス内を分割したら

```
[workspace]
members = [
    "entry",
    "domain",
    "grpc",
    "usecase",
]
resolver = "2"
```

---

# プロジェクト構成の検討 - 過激派？

```
├─productcatalogservice
│  ├─domain
│  │  └─src/...
│  │  └─Cargo.toml
│  ├─entry
│  │  └─src/...
│  │  └─Cargo.toml
│  ├─grpc
│  │  └─src/...
│  │  └─Cargo.toml
│  └─usecase
│     └─src/...
│     └─Cargo.toml
│
...<other service>...
│
├─target
│  ├─debug/...
│  └─...
└─proto
   └─online_boutique.proto
```

---

# まとめ、今後

- cargo workspaceでプロジェクト構成を検討  
- 分割した単位でコンパイルするため、コンパイル時間の短縮につながる
  - 今回は題材が小さかったため問題なかったが、大きくなるとより効果的
- targetは肥大化しがちなのでこちらも効果的
- バージョン管理も共通化できるので試してみる

---