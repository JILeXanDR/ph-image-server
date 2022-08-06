check:
	cargo check

build: test
	cargo build --release

run-prod: build
	./target/release/ph-image-server

test:
	cargo test