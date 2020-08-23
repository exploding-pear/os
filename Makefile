all: fmt analyze test target/debug/bootimage-os.bin
.PHONY: fmt test clean

target/debug/bootimage-os.bin:
	cargo bootimage

fmt:
	cargo fmt

analyze:
	cargo clippy

test:
	cargo test

clean:
	rm -f target/x86_64-os/debug/os
	rm -f target/x86_64-os/debug/bootimage-os.bin