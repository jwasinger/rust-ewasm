WAT_ARGS ?= --fold-exprs --inline-exports --generate-names

SHELL := /bin/bash

all:
	cargo build --target=wasm32-unknown-unknown --release
	cp target/wasm32-unknown-unknown/release/blake2b.wasm blake2b.wasm
	#wasm-gc ecadd-bloated.wasm ecadd.wasm
	#wasm2wat $(WAT_ARGS) blake2.wasm > blake2.wat
	#wasm2wat $(WAT_ARGS) ecadd-bloated.wasm > ecadd-bloated.wat

legit:
	cargo build --release
