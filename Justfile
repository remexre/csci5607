all: check doc build-debug test
clean:
	cargo clean
watch TARGET="all":
	watchexec -cre rs,toml "just {{TARGET}}"

bench:
	cargo +nightly bench --all
build: build-debug build-release
build-debug:
	cargo build --all
build-release:
	cargo build --all --release
check:
	cargo check --all
clippy:
	cargo +nightly clippy --all
doc:
	cargo doc --all
run ASGN +ARGS="":
	cargo run --manifest-path={{ASGN}}/Cargo.toml -- {{ARGS}}
test:
	cargo test --all

build-wasm:
	EMMAKEN_CFLAGS="-s USE_SDL=2" cargo build --target asmjs-unknown-emscripten
	EMMAKEN_CFLAGS="-s USE_SDL=2" cargo build --target wasm32-unknown-emscripten

open-docs:
	cargo doc --open -p common
outdated-deps:
	cargo outdated -R
