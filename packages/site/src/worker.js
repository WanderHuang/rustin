let global = self;

function Event() {
  this.sub = () => console.log('init sub');
}

Event.prototype.register = function (name, fn) {
  global[name] = fn;
  this.dispatch(name);
}

Event.prototype.dispatch = function (action) {
  this.sub(action);
};

Event.prototype.subscribe = function (fn) {
  this.sub = fn;
};

global.ev = new Event();

let readyResolve;
let ready = new Promise((resolve) => (readyResolve = resolve));
let moduleNames = ['bubble', 'quickSort', 'standard', 'wasm'];
let totalNames = [];
global.ev.subscribe((action) => {
  console.log('module is prepared >', action);
  totalNames.push(action);
  if (totalNames.length === moduleNames.length) {
    global.rust_wasm_bindgen.load('wasm/wasm_beginner_bg.wasm').then(data => {
      readyResolve(global.rust_wasm_bindgen.algo)
    })
  }
});

importScripts('js/bubble/index.js');
importScripts('js/quickSort/index.js');
importScripts('js/standard/index.js');
importScripts('wasm/wasm_beginner.js');

ready.then((algo) => {
  console.log('web worker init wasm algorithms >', Object.keys(algo));
  self.postMessage({
    messageType: 'ready',
    messageData: 'worker is prepared',
  });

  function next(messageData) {
    self.postMessage({
      messageType: 'result',
      messageData,
    });
  }

  self.onmessage = (e) => {
    console.log('worker get message > ', e.data);
    let len = Number(e.data.len);
    if (isNaN(len)) {
      return console.log('invalid length', e.data);
    }
    test('冒泡排序', global.js_bubble, algo.bubble)(len).then(next);
    test('快速排序', global.js_quickSort, algo.quickSort)(len).then(next);
    test('语言标准库排序', global.js_standard, algo.standard)(len).then(next);
  };
});

function initArray(len) {
  let arr = new Int32Array(len);
  for (let i = 0; i < len; i++) {
    arr[i] = i;
  }
  arr = shuffle(arr);
  return arr;
}

function shuffle(nums, inplace = false) {
  let random = inplace ? nums : [...nums];
  let i = random.length - 1;
  while (i > 0) {
    let next = (Math.random() * (i + 1)) | 0;
    let temp = random[i];
    random[i] = random[next];
    random[next] = temp;
    i--;
  }
  return random;
}

const test = (name, js, rust) => (len) => {
  return new Promise((resolve) => {
    let statistics = {
      name,
      len,
      timing: [],
      result: [],
    };
    let arr1 = initArray(len);
    let arr2 = initArray(len);

    let jsStart = Date.now();
    let jsResult = js(arr1);
    let jsEnd = Date.now();
    let rustStart = Date.now();
    let rustResult = rust(arr2);
    let rustEnd = Date.now();
    console.log(jsStart, jsEnd, rustStart, rustEnd)

    statistics.timing.push(jsEnd - jsStart);
    statistics.timing.push(rustEnd - rustStart);

    statistics.result.push(jsResult);
    statistics.result.push(rustResult);

    resolve(statistics);
  });
};
