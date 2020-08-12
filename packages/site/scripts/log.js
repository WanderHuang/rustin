const chalk = require('chalk');
const modules = ['build', 'development'];

const levels = [
  {
    lv: 'error',
    color: 'red',
    icon: '❌',
  },
  {
    lv: 'info',
    color: 'white',
    icon: '😼',
  },
  {
    lv: 'warn',
    color: 'yellow',
    icon: '😵',
  },
  {
    lv: 'success',
    color: 'green',
    icon: '✅',
  },
];

function set(obj, key, value) {
  obj[key] = value;
  return obj;
}

const methods = levels.reduce(
  (prev, curr) =>
    set(prev, curr.lv, (...args) =>
      console.log(chalk[curr.color](`${curr.icon} [${curr.lv}]`, ...args))
    ),
  {}
);

const logger = modules.reduce((prev, curr) => {
  prev[curr] = methods;
  return prev;
}, {});

module.exports = logger;
