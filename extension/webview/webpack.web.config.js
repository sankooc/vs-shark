const org = require('./webpack.config.js');
const path = require('path');

module.exports = {
  ...org,
  entry: { 
    main: './src/nav/index.tsx',
    ...org.entry,
  },
  output: {
      filename: './[name].js',
      path: __dirname + "/../../../sankooc.github.io/pcap"
  },
  mode: "production"
};