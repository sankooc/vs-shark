module.exports = {
    entry: {
        app: './src/component/index.tsx',
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
            { test: /\.tsx?$/, loader: "ts-loader" },
            {
                test: /\.svg$/,
                use: ['@svgr/webpack'],
            },
            {
                test: /\.(scss|less|css)$/,
                use: [
                    "style-loader",
                    "css-loader",
                ]
            }, {
                test: /\.(png|jpe?g|gif)$/i,
                use: [
                    {
                        loader: 'file-loader',
                    },
                ],
            },
        ]
    },
    mode: "production"
};