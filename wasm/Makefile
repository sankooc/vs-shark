loc:
	wasm-pack build --target nodejs -d node
web:
	wasm-pack build --target web
test:
	RUST_LOG=debug RUST_BACKTRACE=full cargo test --tests -- --nocapture
clean:
	rm -rf ~/.cargo/registry/index/* ~/.cargo/.package-cache
.PHONY: 