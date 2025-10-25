test:
	RUST_LOG=debug RUST_BACKTRACE=full cargo test tests::basic -- --nocapture
bench:
	RUST_LOG=debug RUST_BACKTRACE=full CRITERION_HOME=./benchmark cargo +nightly bench --bench pcap_bench -- --nocapture
.PHONY: