import md5 from 'md5';

import wasmMd5 from 'wasm-md5';

export default function test(len, count = 1000) {
  return import('wasm-md5').then(algo => {
    let strs = randomString(len);
    let res = {
      strs,
      count,
      jsTime: 0,
      wasmTime: 0,
    };
    let jsRes = "";
    let wasmRes = "";
  
    let i = 0;
    let wasmStart = Date.now();
    while (i < count) {
      wasmRes = algo.rustEncrypt(strs);
      i++;
    }
    res.wasmTime = Date.now() - wasmStart;
  
    i = 0;
    let jsStart = Date.now();
    while (i < count) {
      jsRes = md5(strs);
      i++;
    }
  
    res.jsTime = Date.now() - jsStart;
  
    return {
      ...res,
      jsRes,
      wasmRes
    };
  })
}


function randomString(count = 0) {
  let res = "";

  while(count > 0) {
    if (count < 10) {
      res += cutRandomString(count);
      count = 0;
    } else {
      res += cutRandomString(10);
      count -= 10;
    }
  }

  return res;
}

function cutRandomString(len) {
  return Math.random().toString(16).substring(2, len + 2);
}
