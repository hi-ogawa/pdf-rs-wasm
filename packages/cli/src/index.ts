#!/usr/bin/env node

import process from "node:process";
import { createRequire } from "node:module";
import fs from "node:fs";
import { initSync, PdfParser } from "@hiogawa/pdf-rs-wasm";

export async function main() {
  // argparse
  const [input] = process.argv.slice(2);
  if (!input || !fs.existsSync(input)) {
    console.error("usage: pdf-rs-wasm-cli <input.pdf>");
    process.exit(1);
  }

  // wasm binary file
  const WASM_PATH = require.resolve("@hiogawa/pdf-rs-wasm/pkg/index_bg.wasm");

  // compile wasm
  const wasmSource = await fs.promises.readFile(WASM_PATH);
  const wasmModule = await WebAssembly.compile(wasmSource);

  // initialize rust module
  initSync(wasmModule);

  // run parser
  const inputData = await fs.promises.readFile(input);
  const result = PdfParser.parse_trace(inputData);
  console.log(JSON.stringify(result, null, 2));
}

main();
