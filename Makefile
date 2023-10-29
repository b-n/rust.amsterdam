clean:
	rm -rf bin && mkdir -p bin

# Build the rust sources and make available in bin/rust-amsterdam
build: clean
	cargo build --release && \
	cp ./target/release/rust-amsterdam bin/rust-amsterdam

# Helper to show build artefacts
serve:
	miniserve build/

.PHONY: build clean serve
