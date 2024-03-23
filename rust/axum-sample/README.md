# Axum Sample

<https://docs.rs/axum/latest/axum/index.html>  
ココを見れば大体わかる  

## メモ

[コード](./src/main.rs)を参照　　

- 基本
  - `Router::new().route("/", get(hello))`といった感じで`Router`を作って起動する
    - `"/"`はパス
    - `get(...)`はHTTPメソッド(`post(...)`や`put(...)`など)
      - マクロっぽい
    - HTTPメソッドの`...`には実際の処理をする関数を渡す
      - `hello`は関数。`handler`と呼ばれている
- 細かい話
  - `handler`は0個以上`extractors`を引数に取り、`IntoResponse`の実装を返す関数
  - `extractors`はリクエストパラメータをRustに変換してくれる
    - パスパラメータ、クエリ、フォーム、JSONなんでもござれ
  - `IntoResponse`は自前で実装しなくてもだいたいのものは実装されている
    - `String`、`Result<T, E>`、`StatusCode`、タプル、etc
