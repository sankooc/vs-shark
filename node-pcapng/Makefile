test:
	npx tsc -p ./tsconfig.dev.json --declaration
	NODE_ENV=DETECT node built/test/resolve.js "./sandbox/dns.pcapng"
debug:
	npx tsc -p ./tsconfig.dev.json
	NODE_ENV=DEBUG node built/test/debug.js
compile:
	npx tsc -p ./tsconfig.dev.json --declaration
eslint:
	pnpm eslint src
.PHONY: all test clean