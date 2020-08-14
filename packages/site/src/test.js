function initArray(len) {
  let arr = [];
  for(let i = 0; i < len; i++) {
    arr.push(i);
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

export default (name, js, rust) => (len) => {
  
  return new Promise(resolve => {
    let statistics = {
      name,
      len,
      timing: [],
      result: [],
    };
    let arr1 = initArray(len);
    let arr2 = initArray(len);
  
    let jsStart = performance.now();
    let jsResult = js(arr1);
    let jsEnd = performance.now();
    let rustStart = performance.now();
    let rustResult = rust(arr2);
    let rustEnd = performance.now();
    
    statistics.timing.push(jsEnd - jsStart);
    statistics.timing.push(rustEnd - rustStart);
  
    statistics.result.push(jsResult);
    statistics.result.push(rustResult);


    resolve(statistics)
  });
}