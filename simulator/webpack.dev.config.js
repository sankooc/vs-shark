const org = require('./webpack.config.js');
const path = require('path');

module.exports = {
  ...org,
  devServer: {
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
  }
};