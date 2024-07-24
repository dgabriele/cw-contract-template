network             ?= devnet  # network := devnet|mainnet|testnet
sender              ?= juno16g2rahf5846rxzp3fwlswy08fz8ccuwk03k57y
build_dir           ?= ./builds
tag                 ?= dev
wasm_filename       ?= contract.wasm

# build optimized WASM artifact
build:
	./bin/build

# deploy WASM file (generated from `make build`)
deploy:
	./bin/deploy ./artifacts/$(wasm_filename) $(network) $(sender) $(tag)

# run all unit tests
test:
	RUST_BACKTRACE=1 cargo unit-test

# Generate the contract's JSONSchema JSON files in schemas/
schemas:
	cargo schema

# Run/start local "devnet" validator docker image	
devnet:
	./bin/devnet
