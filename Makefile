all: build-libs build

build-libs:
	cd tidal && cabal build tidal-parse-ffi --enable-static
	mkdir -p lib
	find . -type f -name 'libHStidal-parse-ffi*ghc*.a' -exec cp {} lib \;
	mv -f lib/libHStidal-parse-ffi*ghc*.a lib/libtidalparseffi.a

build:
	cargo build --release

clean:
	cargo clean
	rm -rf Cargo.lock

.PHONY: all build
