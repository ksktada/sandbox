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
