all: fetch-libs build-libs build-app

fetch-libs:
	[ -d Tidal ] || git clone -b tidal-core https://github.com/eilseq/Tidal.git

build-libs:
	cd tidal && cabal build tidal-parse-ffi --enable-static
	mkdir -p lib
	find . -type f -name 'libHStidal-parse-ffi*ghc*.a' -exec cp {} lib \;
	mv -f lib/libHStidal-parse-ffi*ghc*.a lib/libtidalparseffi.a

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
