# pdf-rs-wasm

[wasm-bindgen](https://github.com/rustwasm/wasm-bindgen/) for [pdf-rs](https://github.com/pdf-rs/pdf)

```sh
# development
pnpm i
pnpm build-wasm
pnpm build-cli
pnpm test
```

## npm packages

- https://www.npmjs.com/package/@hiogawa/pdf-rs-wasm
- https://www.npmjs.com/package/@hiogawa/pdf-rs-wasm-cli

## example

```sh
volta install @hiogawa/pdf-rs-wasm-cli
pdf-rs-wasm-cli subtree/pdf/files/example.pdf
```

<details><summary>reveal output</summary>

```json
{
  "pages": [
    {
      "view_box": {
        "x": 0,
        "y": 0,
        "w": 63.5,
        "h": 84.666664
      },
      "items": [
        {
          "type": "Vector",
          "data": {
            "debug": "VectorPath { outline: M 21.68 194 L 158.32 194 L 158.32 220 L 21.68 220 z M 10 10 L 20 20, fill: None, stroke: Some((Solid(0.0, 0.0, 0.972549), 1.0, Stroke { dash_pattern: None, style: StrokeStyle { line_width: 1.0, line_cap: Butt, line_join: Miter(1.0) } })), transform: Transform2F { matrix: Matrix2x2F(<0.35277778, 0, 0, -0.35277778>), vector: Vector2F(F32x2([0.0, 84.666664])) } }"
          }
        },
        {
          "type": "Text",
          "data": {
            "rect": {
              "x": 9.059334,
              "y": 5.6444397,
              "w": 46.101006,
              "h": 8.466667
            },
            "width": 130.68001,
            "bbox": {
              "x": 3.4438403,
              "y": 79.61988,
              "w": 15.806421,
              "h": 2.246109
            },
            "font_size": 24,
            "font": {
              "name": "Times-Roman"
            },
            "text": "Hello World!",
            "chars": [
              {
                "offset": 0,
                "pos": 0,
                "width": 17.328001
              },
              {
                "offset": 1,
                "pos": 17.328001,
                "width": 13.344
              },
              {
                "offset": 2,
                "pos": 30.672,
                "width": 5.328
              },
              {
                "offset": 3,
                "pos": 36,
                "width": 5.328
              },
              {
                "offset": 4,
                "pos": 41.328,
                "width": 13.344
              },
              {
                "offset": 6,
                "pos": 61.343998,
                "width": 22.656
              },
              {
                "offset": 7,
                "pos": 84,
                "width": 13.344
              },
              {
                "offset": 8,
                "pos": 97.344,
                "width": 7.992
              },
              {
                "offset": 9,
                "pos": 105.336,
                "width": 5.328
              },
              {
                "offset": 10,
                "pos": 110.664,
                "width": 13.344
              },
              {
                "offset": 11,
                "pos": 124.008,
                "width": 6.672
              }
            ],
            "color": {
              "type": "Solid",
              "data": [0, 0, 0]
            },
            "alpha": 1,
            "transform": {
              "debug": "Transform2F { matrix: Matrix2x2F(<0.35277778, 0, 0, 0.35277778>), vector: Vector2F(F32x2([9.059334, 14.111107])) }"
            }
          }
        }
      ]
    }
  ]
}
```

</details>
