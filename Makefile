install-rust:
	curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

install:
	cargo install --path .

run:
	cargo run

test:
	cargo install cargo-tarpaulin
	cargo tarpaulin -v
