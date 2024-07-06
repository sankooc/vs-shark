test:
	npx tsc -p ./tsconfig.dev.json --declaration
	NODE_ENV=DETECT node built/test/resolve.js "./sandbox/creden.pcapng"
debug:
	npx tsc -p ./tsconfig.dev.json
	NODE_ENV=DEBUG node built/test/debug.js
compile:
	npx tsc -p ./tsconfig.dev.json --declaration
publish:
	rm -rf built
	npx tsc -p ./tsconfig.json --declaration
	npm publish .
eslint:
	pnpm eslint src
.PHONY: all test clean