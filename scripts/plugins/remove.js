const fs = require('fs');


function RemoveOutputBeforeRun() {}

RemoveOutputBeforeRun.prototype.apply = (compiler) =>  {
  compiler.hooks.beforeRun.tap('RemoveBeforeRun', (compilation) => {
    fs.rmdirSync(compilation.options.output.path, { recursive: true })
    return true
  })
}

module.exports = RemoveOutputBeforeRun;