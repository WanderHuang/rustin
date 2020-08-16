const Server = require('webpack-dev-server');
const webpack = require('webpack');
const path = require('path');
const chalk = require('chalk');
const log = require('./log').development;

// plugin
const HtmlWebpackPlugin = require('html-webpack-plugin');
const CopyWebpackPlugin = require('copy-webpack-plugin');

const cwd = process.cwd();

const config = {
  entry: path.resolve(cwd, 'src/index.js'),
  output: {
    path: path.resolve(cwd, 'dist'),
    filename: 'index.[hash:8].js',
  },
  module: {
    rules: [
      {
        test: /\.css$/i,
        use: ['style-loader', 'css-loader'],
      },
    ],
  },
  plugins: [
    new HtmlWebpackPlugin({
      template: path.resolve(cwd, 'src/index.html'),
    }),
    new CopyWebpackPlugin({
      patterns: [
        {
          from: 'src/**/*',
          transformPath(target, _absolute) {
            if (target.startsWith('src/')) {
              return target.substring(4);
            }
            return target;
          },
        },
        {
          from: 'node_modules/wasm/wasm_beginner*',
          to: 'wasm',
          transformPath(target) {
            return `wasm/${target.substring(22)}`
          }
        }
      ],
    }),
  ],
};

log.info('Compiling...');

const compiler = webpack(config);

const server = new Server(compiler, {
  contentBase: path.join(cwd, 'dist'),
  compress: true,
  hot: true,
});

server.listen(9300, 'localhost', (err) => {
  log.error(err);
});
