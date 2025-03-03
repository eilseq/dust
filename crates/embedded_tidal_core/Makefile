all: fetch-libs build-libs build-app

fetch-libs:
	[ -d Tidal ] || git clone https://github.com/tidalcycles/Tidal.git

build-libs:
	cd tidal && cabal build tidal-parse-ffi --enable-static
	mkdir -p ext
	find . -type f -name 'libHStidal-parse-ffi*ghc*.a' -exec cp {} ext \;
	mv -f ext/libHStidal-parse-ffi*ghc*.a ext/libtidalparseffi.a

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
