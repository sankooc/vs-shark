unit:
	RUST_LOG=debug RUST_BACKTRACE=full cargo test unit -- --nocapture
test:
	RUST_LOG=debug RUST_BACKTRACE=full cargo test tests::real -- --nocapture
clean:
	rm -rf ~/.cargo/registry/index/* ~/.cargo/.package-cache
.PHONY: 