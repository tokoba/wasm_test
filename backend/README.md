# rust + wasm

## installation

wasm-pack は、以下のコマンドでインストールできます。

```bash
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
```

## build

以下のコマンドを実行すると、`pkg` ディレクトリに wasm ファイルが生成されます。

```bash
wasm-pack build --target web
```

## publish

```bash
wasm-pack publish
```
