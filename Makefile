build:
	clear
	cargo build
fmt: 
	cargo fmt
run: fmt
	cargo run
dev:
	cargo watch -x run -w .
