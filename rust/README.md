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

### 並列プログラミング

`spawn`で新しいスレッドが起動する。  
`join()`は全てのスレッドが終了するまで待機する。  

```rust
use std::thread;

let handle = thread::spawn(|| {
  println!("hello from a child thread");
});

let _ = handle.join();
```

スレッドの生存期間が確定できないため変数をそのまま借用させることはできない。  
スレッド間でデータを共有するにはスマートポインタの`Arc`を使用する。  

NG  
```rust
fn process_files_in_parallel(filenames: Vec<String>, glossary: &GigabyteMap) -> io::Result<()>
{
  ...
  for worklist in worklists {
    thread_handles.push(
      spawn(move || process_files(worklist, glossary)) // error
    );
  }
...
}
```

OK  
```rust
use std::sync::Arc;
fn process_files_in_parallel(filenames: Vec<String>, glossary: Arc<GigabyteMap>) -> io::Result<()>
{
  ... 
  for worklist in worklists {
    // ここでの.clone()は、Arcをクローンして参照カウンタを
    // 増やすだけ。GigabyteMapをクローンするわけではない
    let glossary_for_child = glossary.clone();
    thread_handles.push(
      spawn(move || process_files(worklist, &glossary_for_child))
    );
  }
  ...
}
```

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

フューチャーfを受け取り、プールに追加する。  
実行中のスレッドが`block_on`を呼び出すとプールがポーリングされる。  
`async_std::task::JoinHandle`を返す(これもフューチャー)。  
`await`するとでフューチャーの結果が取得できる。  

非同期の場合は`async_std::task::spawn_local(f)`を使用する。  
(↑違うかも)

```rust
async_std::task::block_on()
```

タスクを起動し、その結果で現在のスレッドをブロックする。  
この関数を呼び出すと、非同期タスクが生成されることを除けば  
スレッドを生成してすぐにそのスレッドに参加するのと似ている。  

```mermaid
sequenceDiagram
    main()->>cheapo_request(): cheapo_request()
    cheapo_request()-->>main():A(=Future)
    main()->>A: poll()
    participant tcp as TcpStream::connect()
    cheapo_request()->>tcp: TcpStream::connect()
    tcp-->>cheapo_request(): B(=Future)
    cheapo_request()->>B: poll
```
