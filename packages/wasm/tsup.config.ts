import { defineConfig } from "tsup";

export default defineConfig({
  entry: ["src-js/cli.ts"],
  format: ["esm"],
  dts: true,
});
