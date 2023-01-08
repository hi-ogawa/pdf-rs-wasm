# git subtree

- https://github.com/s3bk/cachelib
- https://github.com/s3bk/pdf_fonts
- https://github.com/pdf-rs/pdf
- https://github.com/pdf-rs/pdf_render

```sh
git subtree add --prefix subtree/cachelib   https://github.com/s3bk/cachelib     master --squash
git subtree add --prefix subtree/pdf_fonts  https://github.com/s3bk/pdf_fonts    master --squash
git subtree add --prefix subtree/pdf        https://github.com/pdf-rs/pdf        master --squash
git subtree add --prefix subtree/pdf_render https://github.com/pdf-rs/pdf_render master --squash
```
