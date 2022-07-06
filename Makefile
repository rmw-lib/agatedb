
SANITIZER_FLAGS=-Zsanitizer=address

upgrade:
	cargo upgrade --workspace

pre-format:
	@rustup component add rustfmt
	@cargo install -q cargo-sort

format: pre-format
	@cargo fmt
	@cargo sort -w ./Cargo.toml ./*/Cargo.toml > /dev/null

clippy:
	cargo clippy --all-targets --all-features --workspace -- -D "warnings"

test:
	cargo test --all-features --workspace

test_sanitizer:
	RUSTFLAGS="$(SANITIZER_FLAGS)" cargo test --all-features --workspace

bench:
	cargo bench --all-features --workspace

bench_sanitizer:
	RUSTFLAGS="$(SANITIZER_FLAGS)" cargo bench --all-features --workspace

dev: format clippy test

cli:
	cd agatedb/adb && cargo build --release

dev_cli:
	cd agatedb/adb && RUST_BACKTRACE=1 watchexec --shell=none -w . -c -r --exts rs,toml --ignore target/ -- cargo run -- ls

clean:
	cargo clean

.PHONY: run clean format clippy test dev
