IP="localhost"

all: blobwar

blobwar: build
	cargo run --bin blobwar

server: build-release
	cargo run --release --bin server

client: build-release
	cargo run --release --bin client $(IP)

build:
	cargo build

build-release:
	cargo build --release