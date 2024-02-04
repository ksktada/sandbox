# Rust

## 参考

プログラミングRust 第2版
<https://www.oreilly.co.jp/books/9784873119786/>

## Install

<https://www.rust-lang.org/ja/tools/install>

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

## メモ

### 非同期プログラミング

なぜ非同期プログラミングをするのか。  
参考より引用(20章冒頭)。  
>複数のプロセッサに仕事を分散させるためにはスレッドが必要で有用だが、スレッドのメモリ消費量は多すぎる。  
>このような場合のために、スレッドを補完する仕事の分割手法が必要になる。  

>非同期タスクを用いると、単一のスレッドもしくは少数のスレッドからなるスレッドプールを用いて、多数の独立した処理を交互に行うことができる。　　
>非同期タスクはスレッドに似ているが、生成に時間がかからず、効率的に処理を引き渡すこと
ができ、メモリのオーバーヘッドはスレッドよりも桁違いに小さい。  

```rust
std::thread::spawn(c)
```

クロージャcを受け取り、それを実行するスレッドを起動する。  
`std::thread::JoinHandle`を返す。  
`join`メソッドはスレッドが終了するまで待ってクロージャcの結果を返す。  

```rust
async_std::task::spawn_local(f)
```

フューチャーfを受け取り、スレッドプールに追加する。  
実行中のスレッドが`block_on`を呼び出すとスレッドプールがポーリングされる。  
`async_std::task::JoinHandle`を返す(これもフューチャー)。  
`await`するとでフューチャーの結果が取得できる。  

非同期の場合は`spawn_local`を使用する。  
