.PHONY: all build

RP=.rp/bin/rp

all:

build:
	$(RP) -- build run wasm-pack build --target web

test:
	$(RP) -- build run cargo test

serve:
	python -m http.server --bind localhost --directory . 8080

