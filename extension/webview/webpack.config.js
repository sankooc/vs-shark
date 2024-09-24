module.exports = {
    entry: {
        // app: "./src/editor/index.tsx",
        app: './src/component/index.tsx',
        // main: './src/nav/index.tsx',
        // tree: './src/tree/index.tsx',
        // hex: './src/hex/index.tsx',
        // shared: 'react',
    },
    output: {
        filename: './[name].js',
        path: __dirname + "/../media"
    },
    resolve: {
        extensions: [".ts", ".tsx", ".js", ".json", ".less", ".svg"]
    },
    module: {
        rules: [
            { test: /\.tsx?$/, loader: "awesome-typescript-loader" },
            {
                test: /\.svg$/,
                loader: 'svg-react-loader'
            },
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