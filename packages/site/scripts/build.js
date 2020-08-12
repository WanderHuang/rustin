const webpack = require('webpack');
const path = require('path');
const chalk = require('chalk');
const log = require('./log').build;

// plugin
const HtmlWebpackPlugin = require('html-webpack-plugin');

const cwd = process.cwd();

log.info(`building in ${cwd}`);

const config = {
  entry: path.resolve(cwd, './src/index.js'),
  output: {
    path: path.resolve(cwd, 'dist'),
    filename: 'index.[hash:8].js'
  },
  plugins: [
    new HtmlWebpackPlugin()
  ]
};

const compiler = webpack(config);

compiler.run((err, stats) => {
  if (err || stats.hasErrors()) {
    log.error(err || stats.toJson().errors);
  } else {
    const res = stats.toJson();
    log.info('Time(s)', res.time);
    log.success('build finished');
  }
});
