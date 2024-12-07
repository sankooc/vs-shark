pre:
	cd constants && node rust.js
web-site:
	cp extension/CHANGELOG.md doc/pages/
wasm: pre web-site
	cd crates/wasm && make web
wasm-debug:
	cd crates/wasm && make web-debug
web-demo: wasm
	cd extension/webview && rm -rf dist_web && npm run reset && npm run gen && npm run css && npm run build-web
view: wasm
	cd extension/webview && npm run reset && npm run css && npm run build
serve:
	cd extension/webview && npm run reset && npm run serve
clean-web:
	cd extension/webview && rm -rf node_modules && rm src/gen.ts
clean:
	rm -rf ~/.cargo/registry/index/* ~/.cargo/.package-cache
.PHONY: wasm