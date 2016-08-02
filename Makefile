
all: run 


run: debug
	cargo run

release:
	cargo build --release

debug:
	cargo build

test:
	cargo test


