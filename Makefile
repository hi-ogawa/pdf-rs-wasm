# auto generate phony targets
.PHONY: $(shell grep --no-filename -E '^([a-zA-Z_-]|/)+:' $(MAKEFILE_LIST) | sed 's/:.*//')

patch:
	git -C thirdparty/cachelib apply < thirdparty/cachelib.diff
	git -C thirdparty/pdf apply < thirdparty/pdf.diff

patch/clean:
	git -C thirdparty/cachelib restore .
	git -C thirdparty/pdf restore .

patch/create:
	git -C thirdparty/cachelib diff > thirdparty/cachelib.diff
	git -C thirdparty/pdf diff > thirdparty/pdf.diff
