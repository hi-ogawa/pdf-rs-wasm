# pdf-rs-wasm

wasm-bindgen for [pdf-rs](https://github.com/pdf-rs/pdf).

```sh
git submodule update --init
make patch

pnpm i
pnpm build
pnpm test

cd thidparty/pdf_render
STANDARD_FONTS=../pdf_fonts cargo run -p pdf_render --example trace -- ../pdf/files/example.pdf
```
