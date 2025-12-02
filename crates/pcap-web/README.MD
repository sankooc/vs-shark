# pcap-web

HTTP service to explore a capture file (pcap/pcapng) with the same data model used in the VS Code extension / plugin. After you point the service to a file, you can browse frames, protocols, statistics and conversations via simple REST endpoints.

## Quick Start
```bash
cargo run -p pcap-web -- --file /path/to/capture.pcap
# then open http://127.0.0.1:8080
```

## Core Idea
1. Provide a file path via CLI.
2. Backend parses packets using internal crates (`pcap`, `util`).
3. Data becomes accessible through HTTP routes (similar to in-editor views).


## Extending
Add new protocol views by:
1. Enhancing parsing in `crates/parser`
2. Creating a route module in `src/routes/`
3. Registering it in `src/routes/mod.rs`

## License
MIT