# Game by Wasm

備忘のためメモしておく。

## 参考

RustとWebAssemblyによるゲーム開発  
https://www.oreilly.co.jp/books/9784814400393/

## 事前準備

- Node.jsをインストールしておく。
- wasm-packをインストールしておく。
  - LinuxやMacの場合はコマンドでインストールできる。
    - ```sh
        curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
        ```
  - Windowsの場合は下記からインストーラをダウンロードしてインストールする。
    - https://rustwasm.github.io/

## プロジェクト作成

コマンドで準備できる。

```sh
npm init rust-webpack <app_name>
cd <app_name>
npm install
```

起動する。

```sh
npm run start
```

ブラウザのデベロッパーツールのコンソールに「Hello World」と表示されたら成功。

## 1章メモ

```rust
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
```

`wasm_bindgen...`はJavaScriptから`main_js`を呼び出せるようエクスポートする。  
Rustは`Result`をEthierっぽい感じで結果を表現している(Leftが失敗、Rightが成功)。  


```rust
let window = web_sys::window().unwrap();
let document = window.document().unwrap();
```

JavaScriptの`Window`と`Document`を取得している。  

```rust
let canvas = document
    .get_element_by_id("canvas")
    .unwrap()
    .dyn_into::<web_sys::HtmlCanvasElement>()
    .unwrap();

let context = canvas
    .get_context("2d")
    .unwrap()
    .unwrap()
    .dyn_into::<web_sys::CanvasRenderingContext2d>()
    .unwrap();
```

`Document`からid指定で`Element`(DOM要素)を取得している。  
`dyn_into`は`HtmlCanvasElement`にキャストしている。  
`get_element_by_id`の返り値は`Element`型であり`HtmlCanvasElement`ではない。  
`Element`のままだと`get_context`をコールできない。  
(JavaScriptは動的型付け言語のためオブジェクトにメソッドがあればコールできる(なければ例外発生))  

`draw_triangle`は`CanvasRenderingContext2D`のメソッドを使って三角形を描画している。  

## 2章メモ

```rust
let image = web_sys::HtmlImageElement::new().unwrap();
image.set_src("Idle (1).png");
context.draw_image_with_html_image_element(&image, 0.0, 0.0);
```

`HtmlImageElement`は画像用の`Element`(つまりimageタグ)。  
上記だけでは描画できない。  
画像のロードを待つ必要がある。  

```rust
wasm_bindgen_futures::spawn_local(async move {
    let (success_tx, success_rx) = futures::channel::oneshot::channel::<Result<(), JsValue>>();
    let success_tx = Rc::new(Mutex::new(Some(success_tx)));
    let error_tx = Rc::clone(&success_tx);
    let image = web_sys::HtmlImageElement::new().unwrap();
    let callback = Closure::once(move || {
        if let Some(success_tx) = success_tx.lock().ok().and_then(|mut opt| opt.take()) {
            success_tx.send(Ok(()));
        }
    });
    let error_callback = Closure::once(move |err| {
        if let Some(error_tx) = error_tx.lock().ok().and_then(|mut opt| opt.take()) {
            error_tx.send(Err(err));
        }
    });
    image.set_onload(Some(callback.as_ref().unchecked_ref()));
    image.set_onerror(Some(error_callback.as_ref().unchecked_ref()));
    callback.forget();
    image.set_src("Idle (1).png");
    success_rx.await;
    context.draw_image_with_html_image_element(&image, 0.0, 0.0);

});
```

とても長い。。。  
`wasm_bindgen_futures::spawn_local`は非同期関数(`Future`を返す関数)を実行するために必要な関数である。  
(spawn...コンピュータの文脈では「サブプロセスを**生成する**」を意味する)  

```mermaid


```

