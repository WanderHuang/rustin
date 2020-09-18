const webpack = require('webpack');
const fs = require('fs');
const path = require('path');
const RemoveOutputBeforeRun = require('./plugins/remove');

const basePath = process.cwd();
const srcPath = path.resolve(basePath, 'src');
const workerPath = path.resolve(srcPath, 'workers');
let workers = fs.readdirSync(workerPath)
  .concat(path.resolve(srcPath, 'index.js'))
  .map(entry => ({ path: path.resolve(srcPath, 'workers', entry), name: path.basename(entry, '.js') }))
  .reduce((prev, curr) => {
    prev[curr.name] = curr.path;
    return prev;
  }, {});
console.log(workers)

webpack(
  {
    entry: workers,
    output: {
      filename: '[name].js',
      path: path.resolve(basePath, 'dist')
    },
    mode: 'production',
    module: {
      rules: [
        {
          use: {
            loader: 'babel-loader',
            options: {
              presets: ['@babel/preset-env']
            }
          }
        }
      ]
    },
    plugins: [
      new RemoveOutputBeforeRun()
    ]
  }
  ,
  (err, stat) => {
    if (err) {
      console.log(err);
    }

    if (stat.hasErrors())  {
      console.log(stat.toJson().errors)
    }

    console.log('[ build ] finished')
  }
);