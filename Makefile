all: fetch-libs build-libs build-app

fetch-libs:
	[ -d Tidal ] || git clone -b tidal-core https://github.com/eilseq/Tidal.git

build-libs:
	cd crates/evaluator && make build-libs
	
build-app:
	cargo build --release

run:
	cargo run

clean:
	cargo clean
	rm -rf lib
	rm -rf tidal
	rm -rf Cargo.lock

.PHONY: all
