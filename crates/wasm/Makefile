loc:
	rm -rf build/node    && RUST_LOG=debug wasm-pack build --target nodejs -d build/node
web:
	rm -rf build/browser && RUST_LOG=debug wasm-pack build --target web    -d build/browser
clean:
	rm -rf ~/.cargo/registry/index/* ~/.cargo/.package-cache
.PHONY: 