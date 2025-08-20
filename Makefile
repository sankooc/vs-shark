pre:
	cd constants && node rust.js
web-site:
	cp extension/CHANGELOG.md doc/pages/
wasm:
	cd crates/wasm && make web
clean:
	rm -rf ~/.cargo/registry/index/* ~/.cargo/.package-cache
re-intall:
	rm $(which wasm-pack) && curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
.PHONY: wasm