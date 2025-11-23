pre:
	cd constants && node rust.js
web-site:
	cp extension/CHANGELOG.md doc/pages/
wasm:
	cd crates/wasm && make web
web-demo: wasm
	cd webview && rm -rf dist && npm run ins && npm run build:local
clean:
	rm -rf ~/.cargo/registry/index/* ~/.cargo/.package-cache
coverage:
	cargo tarpaulin -p pcap  --ignore-tests --out Html --output-dir doc/public/coverage
re-intall:
	rm $(which wasm-pack) && curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
.PHONY: wasm