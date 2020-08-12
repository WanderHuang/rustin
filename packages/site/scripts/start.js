const Server = require('webpack-dev-server');
const webpack = require('webpack');
const path = require('path');
const chalk = require('chalk');
const log = require('./log').development;

// plugin
const HtmlWebpackPlugin = require('html-webpack-plugin');

const cwd = process.cwd();

const config = {
  entry: path.resolve(cwd, './src/index.js'),
  output: {
    path: path.resolve(cwd, './dist'),
  },
  plugins: [
    new HtmlWebpackPlugin()
  ],
};

log.info('Compiling...');

const compiler = webpack(config);

const server = new Server(compiler, {
  contentBase: path.join(cwd, './dist'),
  compress: true,
  hot: true
});

server.listen(9300, 'localhost', err => {
  log.error(err)
})


