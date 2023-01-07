# pdf-rs-wasm

[wasm-bindgen](https://github.com/rustwasm/wasm-bindgen/) for [pdf-rs](https://github.com/pdf-rs/pdf).
the goal is to be a robust replacement of [pdf2json](https://github.com/modesty/pdf2json) on javascript ecosystem.

```sh
git submodule update --init
make patch
make hack-wasm-bindgen-test-esm

pnpm i
pnpm build
pnpm test
```
