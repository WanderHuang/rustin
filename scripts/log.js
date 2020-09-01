const chalk = require('chalk');

function std() {
  console.log(chalk.cyan('[clay]'), ...arguments);
}

std.err = function() {
  console.log(chalk.red('[clay]'), ...arguments);
}

std.welcome = function() {
  console.log(chalk.green('[clay] ^_^ < Thanks for using \`clay\` to start your \`Wasm-Project\` > ^_^'))
}


module.exports.std = std;
