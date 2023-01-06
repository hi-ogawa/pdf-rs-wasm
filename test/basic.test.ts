import { beforeAll, describe, expect, it } from "vitest";
import { initSync, PdfParser } from "../pkg/index";
import fs from "node:fs";

beforeAll(async () => {
  const wasmSource = await fs.promises.readFile("pkg/index_bg.wasm");
  const wasmModule = await WebAssembly.compile(wasmSource);
  initSync(wasmModule);
});

describe("PdfParser", () => {
  it("parse_operations", async () => {
    const data = await fs.promises.readFile("thirdparty/pdf/files/example.pdf");
    const result = PdfParser.parse_operations(data);
    expect(result).toMatchInlineSnapshot(`
      {
        "pages": [
          {
            "operations": [
              "FillColorSpace { name: Name(\\"DeviceRGB\\") }",
              "StrokeColorSpace { name: Name(\\"DeviceRGB\\") }",
              "StrokeColor { color: Other([Integer(0), Integer(0), Number(0.972549)]) }",
              "Rect { rect: Rect { x: 21.68, y: 194.0, width: 136.64, height: 26.0 } }",
              "MoveTo { p: Point { x: 10.0, y: 10.0 } }",
              "LineTo { p: Point { x: 20.0, y: 20.0 } }",
              "Stroke",
              "BeginText",
              "TextFont { name: Name(\\"F0\\"), size: 24.0 }",
              "MoveTextPosition { translation: Point { x: 25.68, y: 200.0 } }",
              "TextDraw { text: \\"Hello World!\\" }",
              "EndText",
            ],
          },
        ],
      }
    `);
  });

  it("parse_trace", async () => {
    const data = await fs.promises.readFile("thirdparty/pdf/files/example.pdf");
    const result = PdfParser.parse_trace(data);
    expect(result).toMatchInlineSnapshot(`
      [
        "Vector(VectorPath { outline: M 21.68 194 L 158.32 194 L 158.32 220 L 21.68 220 z M 10 10 L 20 20, fill: None, stroke: Some((Solid(0.0, 0.0, 0.972549), 1.0, Stroke { dash_pattern: None, style: StrokeStyle { line_width: 1.0, line_cap: Butt, line_join: Miter(1.0) } })), transform: Transform2F { matrix: Matrix2x2F(<0.35277778, 0, 0, -0.35277778>), vector: Vector2F(F32x2([0.0, 84.666664])) } })",
        "Text(TextSpan { rect: RectF(<9.059334, 5.6444397, 55.16034, 14.111107>), width: 130.68001, bbox: Some(RectF(<3.4438403, 79.61988, 19.250261, 81.86599>)), font_size: 24.0, text: \\"Hello World!\\", chars: [TextChar { offset: 0, pos: 0.0, width: 17.328001 }, TextChar { offset: 1, pos: 17.328001, width: 13.344 }, TextChar { offset: 2, pos: 30.672, width: 5.328 }, TextChar { offset: 3, pos: 36.0, width: 5.328 }, TextChar { offset: 4, pos: 41.328, width: 13.344 }, TextChar { offset: 6, pos: 61.343998, width: 22.656 }, TextChar { offset: 7, pos: 84.0, width: 13.344 }, TextChar { offset: 8, pos: 97.344, width: 7.992 }, TextChar { offset: 9, pos: 105.336, width: 5.328 }, TextChar { offset: 10, pos: 110.664, width: 13.344 }, TextChar { offset: 11, pos: 124.008, width: 6.672 }], color: Solid(0.0, 0.0, 0.0), alpha: 1.0, transform: Transform2F { matrix: Matrix2x2F(<0.35277778, 0, 0, 0.35277778>), vector: Vector2F(F32x2([9.059334, 14.111107])) } })",
      ]
    `);
  });
});
