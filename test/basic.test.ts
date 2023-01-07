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
              {
                "debug": "FillColorSpace { name: Name(\\"DeviceRGB\\") }",
              },
              {
                "debug": "StrokeColorSpace { name: Name(\\"DeviceRGB\\") }",
              },
              {
                "debug": "StrokeColor { color: Other([Integer(0), Integer(0), Number(0.972549)]) }",
              },
              {
                "debug": "Rect { rect: Rect { x: 21.68, y: 194.0, width: 136.64, height: 26.0 } }",
              },
              {
                "debug": "MoveTo { p: Point { x: 10.0, y: 10.0 } }",
              },
              {
                "debug": "LineTo { p: Point { x: 20.0, y: 20.0 } }",
              },
              {
                "debug": "Stroke",
              },
              {
                "debug": "BeginText",
              },
              {
                "debug": "TextFont { name: Name(\\"F0\\"), size: 24.0 }",
              },
              {
                "debug": "MoveTextPosition { translation: Point { x: 25.68, y: 200.0 } }",
              },
              {
                "debug": "TextDraw { text: \\"Hello World!\\" }",
              },
              {
                "debug": "EndText",
              },
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
        [
          {
            "Vector": {},
          },
          {
            "Text": {
              "alpha": 1,
              "font_size": 24,
              "rect": {
                "x": 9.059334,
                "y": 5.6444397,
              },
              "text": "Hello World!",
              "width": 130.68001,
            },
          },
        ],
      ]
    `);
  });
});
