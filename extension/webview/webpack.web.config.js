const org = require('./webpack.config.js');
const WorkboxPlugin = require('workbox-webpack-plugin');
const path = require('path');

module.exports = {
  ...org,
  entry: { 
    main: './src/nav/index.tsx',
    ...org.entry,
  },
  output: {
      filename: './[name].js',
      path: __dirname + "/dist_web",
      // path: __dirname + "/../../../sankooc.github.io/pcap"
  },
  plugins: [
    new WorkboxPlugin.GenerateSW({
      clientsClaim: true,
      skipWaiting: true
    })
  ],
  mode: "production"
};