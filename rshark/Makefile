node:
	wasm-pack build --target nodejs
web:
	wasm-pack build --target web
test:
	RUST_LOG=debug cargo test -- --nocapture
clean:
	rm -rf ~/.cargo/registry/index/* ~/.cargo/.package-cache
.PHONY: 