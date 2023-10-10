import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";
import init, { PdfParser } from "../pkg";
import { TinyCliCommand, arg, tinyCliMain } from "@hiogawa/tiny-cli";
import { name as packageName, version as packageVersion } from "../package.json";

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const WASM_PATH = path.join(__dirname, "..", "pkg", "index_bg.wasm");

const cli = new TinyCliCommand(
  {
    program: packageName.split("/")[1],
    version: packageVersion,
    args: {
      infile: arg.string("input pdf file", { positional: true }),
      password: arg.string("optional password", { optional: true }),
    },
  },
  async ({ args }) => {
    // init wasm
    const wasmSource = await fs.promises.readFile(WASM_PATH);
    const wasmModule = await WebAssembly.compile(wasmSource);
    await init(wasmModule);

    // run parser
    const inputData = await fs.promises.readFile(args.infile);
    const result = PdfParser.parse_operations(inputData, args.password);
    console.log(JSON.stringify(result, null, 2));
  }
);

tinyCliMain(cli);
