const org = require('./webpack.config.js');
const path = require('path');

module.exports = {
  ...org,
  entry: { 
    main: './src/nav/index.tsx',
    ...org.entry,
    react: 'react',
    reactDom: 'react-dom',
    // primeflex: 'primeflex',
    // primeicons: 'primeicons/primeicons.css',

    // primereact: 'primereact',
  },
  devServer: {
    client: {
      overlay: {
        errors: true,
        warnings: false,
        runtimeErrors: true,
      },
    },
    static: [{
      directory: path.join(__dirname, 'public'),
    },
    {
      directory: path.join(__dirname, 'dist'),
      publicPath: '/dist',
    }],
    compress: true,
    port: 9000,
  },
  devtool: 'source-map',
  watchOptions: {
    ignored: /node_modules/,
  },
  mode: 'development',
  externals: {
      "echarts": "echarts",
      "bootstrap": "bootstrap",
      // "react": "React",
      // "react-dom": "react-dom",
  },
};