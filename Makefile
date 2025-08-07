1:
	mkdir -p app/assets
	cd app; trunk watch
2:
	cargo watch -w server -w src -w config.json -x "run"

build:
	mkdir -p app/assets
	cd app; trunk build
	cargo build

serve: build
	cargo run

install:
	curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
	cargo install trunk
	cargo install cargo-watch
