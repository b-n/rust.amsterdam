ARCH=x86_64-unknown-linux-gnu

clean:
	rm -rf bin && mkdir -p bin

# Build the rust sources and make available in bin/rust-amsterdam
build: clean
	cargo build --release --target ${ARCH} && \
	cp ./target/${ARCH}/release/rust-amsterdam bin/rust-amsterdam

# Helper to show build artefacts
serve:
	miniserve build/

.PHONY: build clean serve
