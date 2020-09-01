const utils = require('./utils');
const chalk = require('chalk');
const { std } = require('./log');

const system = utils.getSystem();

std.welcome();
std(`Your system is \`${system.machine}\``)

if (utils.cmdExists('wasm-pack')) {
  utils.exec('wasm-pack build --release')
} else {
  process.exit(0);
}