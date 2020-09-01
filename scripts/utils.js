const os = require('os');
const spawnSync = require('child_process').spawnSync;
const execSync = require('child_process').execSync;
const { std } = require('./log');


module.exports.getSystem = function () {
  return {
    machine: os.platform(),
    cpus: os.cpus(),
    eol: os.EOL,
  }
}


module.exports.cmdExists = function (cmd) {
  if (typeof cmd !== 'string') return false;
  let res = spawnSync(cmd, ['/?'], { encoding: 'utf8' });
  if (res.error) {
    std.err(`cannot find command \`${cmd}\`, maybe you should install it at first`)
    return false;
  }
  return true
}

module.exports.exec = function(cmd) {
  return execSync(cmd);
}
