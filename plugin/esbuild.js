const esbuild = require("esbuild");
const fs = require('fs');
const path = require('path');

const production = process.argv.includes('--production');
const watch = process.argv.includes('--watch');

/**
 * @type {import('esbuild').Plugin}
 */
const esbuildProblemMatcherPlugin = {
	name: 'esbuild-problem-matcher',
	setup(build) {
		build.onStart(() => {
			console.log('[watch] build started');
		});
		build.onEnd((result) => {
			result.errors.forEach(({ text, location }) => {
				console.error(`✘ [ERROR] ${text}`);
				console.error(`    ${location.file}:${location.line}:${location.column}:`);
			});
			console.log('[watch] build finished');
		});
	},
};

/**
 * @type {import('esbuild').Plugin}
 */
const copyWasmPlugin = {
	name: 'copy-wasm',
	setup(build) {
		build.onEnd(() => {
			if (!fs.existsSync('dist')) {
				fs.mkdirSync('dist', { recursive: true });
			}
			const wasmSourcePath = path.resolve(__dirname, 'node_modules/.pnpm/wasm-pcps@file+..+crates+wasm+node/node_modules/wasm-pcps/wasm_pcps_bg.wasm');
			const wasmDestPath = path.resolve(__dirname, 'dist/wasm_pcps_bg.wasm');
			
			if (fs.existsSync(wasmSourcePath)) {
				fs.copyFileSync(wasmSourcePath, wasmDestPath);
				console.log('✓ Copied WASM file to dist directory');
			} else {
				console.error('✘ WASM file not found:', wasmSourcePath);
			}
		});
	},
};

async function main() {
	const ctx = await esbuild.context({
		entryPoints: [
			'src/extension.ts'
		],
		bundle: true,
		format: 'cjs',
		minify: production,
		sourcemap: !production,
		sourcesContent: false,
		platform: 'node',
		outfile: 'dist/extension.js',
		external: ['vscode'],
		logLevel: 'silent',
		plugins: [
			copyWasmPlugin,
			esbuildProblemMatcherPlugin,
		],
	});
	if (watch) {
		await ctx.watch();
	} else {
		await ctx.rebuild();
		await ctx.dispose();
	}
}

main().catch((err) => {
	console.error(err);
	process.exit(1);
});
