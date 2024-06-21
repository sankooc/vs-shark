module.exports = {
  entry: "./src/index.tsx",
  output: {
      filename: "bundle.js",
      path: __dirname + "/dist"
  },
  resolve: {
      // Add '.ts' and '.tsx' as resolvable extensions.
      extensions: [".ts", ".tsx", ".js", ".json", ".less"]
  },
  module: {
      rules: [
          // All files with a '.ts' or '.tsx' extension will be handled by 'awesome-typescript-loader'.
          { test: /\.tsx?$/, loader: "awesome-typescript-loader" },
          {
            test: /\.(scss|less|css)$/,
            use: [
                "style-loader",
                "css-loader",
            ]
        },
          // All output '.js' files will have any sourcemaps re-processed by 'source-map-loader'.
          // { enforce: "pre", test: /\.js$/, loader: "source-map-loader" }
      ]
  },
  mode: "production"
  // externals: {
  //     "react": "React",
  //     "react-dom": "ReactDOM"
  // },
};