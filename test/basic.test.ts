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
    const data = await fs.promises.readFile("subtree/pdf/files/example.pdf");
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
    const data = await fs.promises.readFile("subtree/pdf/files/example.pdf");
    const result = PdfParser.parse_trace(data);
    expect(result).toMatchInlineSnapshot(`
      {
        "pages": [
          {
            "items": [
              {
                "Vector": {},
              },
              {
                "Text": {
                  "alpha": 1,
                  "bbox": {
                    "h": 2.246109,
                    "w": 15.806421,
                    "x": 3.4438403,
                    "y": 79.61988,
                  },
                  "font_size": 24,
                  "rect": {
                    "h": 8.466667,
                    "w": 46.101006,
                    "x": 9.059334,
                    "y": 5.6444397,
                  },
                  "text": "Hello World!",
                  "width": 130.68001,
                },
              },
            ],
          },
        ],
      }
    `);
  });
});
