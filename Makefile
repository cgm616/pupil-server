all: frontbuild run

build: 
	cargo build

run: build
	cargo run

test: 
	export RUST_TEST_THREADS=1
	export RUST_BACKTRACE=1
	cargo test

frontbuild:
	cd ./frontend && npm run build

frontrun:
	cd ./frontend && npm start

