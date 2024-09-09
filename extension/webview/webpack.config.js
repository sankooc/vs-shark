module.exports = {
    entry: { 
        // app: "./src/editor/index.tsx",
        app: './src/app/index.tsx',
        // main: './src/nav/index.tsx',
        // tree: './src/tree/index.tsx',
        // hex: './src/hex/index.tsx',
        // shared: 'react',
    },
    output: {
        filename: './[name].js',
        path: __dirname + "/dist"
    },
    resolve: {
        extensions: [".ts", ".tsx", ".js", ".json", ".less"]
    },
    module: {
        rules: [
            { test: /\.tsx?$/, loader: "awesome-typescript-loader" },
            {
                test: /\.(scss|less|css)$/,
                use: [
                    "style-loader",
                    "css-loader",
                ]
            },
        ]
    },
    mode: "production"
};