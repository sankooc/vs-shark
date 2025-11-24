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
gui-demo:
	cargo tauri build --target x86_64-unknown-linux-gnu
	@for f in target/x86_64-unknown-linux-gnu/release/bundle/deb/pcapviewer_gui_*_amd64.deb; do \
		[ -f "$$f" ] && echo "Installing $$f ..." && sudo dpkg -i "$$f" && sudo apt-get install -f -y; \
	done
re-intall:
	rm $(which wasm-pack) && curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
gui-uninstall:
	sudo dpkg -r pcapviewer-gui 
.PHONY: wasm