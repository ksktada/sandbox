# Axum Sample

## メモ

[コード](./src/main.rs)を参照　　

- `Router::new().route("/", get(hello))`といった感じで`Router`を作って起動する
  - `"/"`はパス
  - `get(...)`はHTTPメソッド(`post(...)`や`put(...)`など)
  - HTTPメソッドの`...`には実際の処理をする関数を渡す
    - `hello`は関数
