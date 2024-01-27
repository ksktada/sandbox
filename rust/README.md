# Rust

## Install

https://www.rust-lang.org/ja/tools/install

## Cargo

### New project

```sh
cargo new <package_name>
```

### Run

```sh
cargo run
```

## Update

```sh
rustup update
```

## Tips

### JSON

[json_to_struct](./json_to_struct)

`products.json` is from [here](https://github.com/GoogleCloudPlatform/microservices-demo).  

### Workspace

- 依存関係について
  - 外部クレートは複数の内部クレートで共有される(1つに解決される)
    - Cargo.lockは内部クレートで共存する
    - Cargoがよしなに解決してくれる
  - ただしそれぞれのクレートのCargo.tomlの`[dependencies]`に外部クレートは指定する必要あり