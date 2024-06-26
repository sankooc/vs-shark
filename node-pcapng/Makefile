clean-ts:
	rm -rf built
clean-temp:
	rm -rf temp
	mkdir -p temp
clear: clean-ts clean-temp
test: clear
	npx tsc -p ./tsconfig.dev.json --declaration
	NODE_ENV=DETECT node built/test/resolve.js "./sandbox/11.pcapng"
debug: clean-ts
	npx tsc -p ./tsconfig.dev.json
	NODE_ENV=DEBUG node built/test/debug.js
compile: clean-ts
	npx tsc -p ./tsconfig.dev.json --declaration
eslint:
	pnpm eslint src