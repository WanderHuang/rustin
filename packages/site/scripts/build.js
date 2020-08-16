const webpack = require('webpack');
const path = require('path');
const chalk = require('chalk');
const log = require('./log').build;

// plugin
const HtmlWebpackPlugin = require('html-webpack-plugin');
const CopyWebpackPlugin = require('copy-webpack-plugin');
const { CleanWebpackPlugin } = require('clean-webpack-plugin');

const cwd = process.cwd();

log.info(`building in ${cwd}`);

const config = {
  entry: path.resolve(cwd, 'src/index.js'),
  output: {
    path: path.resolve(cwd, 'dist'),
    filename: 'index.[hash:8].js',
  },
  module: {
    rules: [
      {
        test: /\.css$/,
        use: ['style-loader', 'css-loader'],
      },
    ],
  },
  plugins: [
    new CleanWebpackPlugin(),
    new HtmlWebpackPlugin({
      template: path.resolve(cwd, './src/index.html'),
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
