TIDAL_PARSE_DIR = tidal-parse

all: build-libs build

build-libs:
	cd ${TIDAL_PARSE_DIR} && cabal build --enable-static
	mkdir -p lib
	find . -type f -name 'libHStidal-parse*ghc*.a' -exec cp {} lib \;
	mv -f lib/libHStidal-parse*ghc*.a lib/libtidalparse.a

build:
	cargo build --release

clean:
	cargo clean
	rm -rf Cargo.lock

.PHONY: all build
