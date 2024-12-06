loc:
	RUST_LOG=info wasm-pack build --target nodejs -d node
web:
	rm -rf pkg && wasm-pack build --target web
web-debug:
	rm -rf pkg && wasm-pack build --target web --debug
test:
	RUST_LOG=debug RUST_BACKTRACE=full cargo test --tests -- --nocapture
clean:
	rm -rf ~/.cargo/registry/index/* ~/.cargo/.package-cache
.PHONY: 