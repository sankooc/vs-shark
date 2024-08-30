pre:
	cd constants && node rust.js
wasm:
	cd wasm && make web
view:
	cd webview && npm run reset && npm run css && npm run build
serve:
	cd webview && npm run reset && npm run serve
clean:
	rm -rf ~/.cargo/registry/index/* ~/.cargo/.package-cache
.PHONY: wasm