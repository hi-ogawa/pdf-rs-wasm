#!/bin/bash
set -eu -o pipefail

# remove unnecessary auto-generated files
rm -f pkg/package.json pkg/.gitignore pkg/README.md

# rename to mjs
mv pkg/index.js pkg/index.mjs

# transpile to cjs
npx babel --plugins @babel/plugin-transform-modules-commonjs pkg/index.mjs --out-file pkg/index.cjs

# fix cjs
sed -i 's/import.meta.url/new Error("import.meta.url")/' pkg/index.cjs
