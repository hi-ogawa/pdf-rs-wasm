# pdf-rs-wasm

wasm-bindgen for [pdf-rs](https://github.com/pdf-rs/pdf).

```sh
git submodule update --init
git -C thirdparty/cachelib apply < thirdparty/cachelib.diff
git -C thirdparty/pdf apply < thirdparty/pdf.diff

pnpm i
pnpm build
pnpm test

# update diff
git -C thirdparty/cachelib diff > thirdparty/cachelib.diff
git -C thirdparty/pdf diff > thirdparty/pdf.diff
```
