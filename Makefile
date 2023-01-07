# auto generate phony targets
.PHONY: $(shell grep --no-filename -E '^([a-zA-Z_-]|/)+:' $(MAKEFILE_LIST) | sed 's/:.*//')

patch:
	git -C thirdparty/cachelib apply < thirdparty/cachelib.diff
	git -C thirdparty/pdf apply < thirdparty/pdf.diff
	git -C thirdparty/pdf_render apply < thirdparty/pdf_render.diff

patch/clean:
	git -C thirdparty/cachelib restore .
	git -C thirdparty/pdf restore .
	git -C thirdparty/pdf_render restore .
	git -C thirdparty/pdf_fonts restore .

patch/create:
	git -C thirdparty/cachelib diff > thirdparty/cachelib.diff
	git -C thirdparty/pdf diff > thirdparty/pdf.diff
	git -C thirdparty/pdf_render diff > thirdparty/pdf_render.diff

# force target/wasm32-unknown-unknown/wbg-tmp/run.js to be treated as commonjs
hack-wasm-bindgen-test-esm:
	mkdir -p target
	echo '{ "module": "commonjs" }' > target/package.json
