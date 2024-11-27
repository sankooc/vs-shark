pre:
	cd constants && node rust.js
wasm: pre
	cd wasm && make web && make loc
web-demo:
	cd extension/webview && rm -rf dist_web && npm run reset && npm run css && npm run build-web
view: wasm
	cd extension/webview && npm run reset && npm run css && npm run build
serve:
	cd extension/webview && npm run reset && npm run serve
clean:
	rm -rf ~/.cargo/registry/index/* ~/.cargo/.package-cache
.PHONY: wasm