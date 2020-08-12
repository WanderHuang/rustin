import('a_wasm').then(module => {
  let arr = [];

  let len = 10000;

  for(let i = 0; i < len; i++) {
    arr.push(i);
  }

  arr = shuffle(arr);

  let a1 = [...arr];
  let a2 = [...arr];
  console.log('start bubble', len)

  console.time('js')
  a1 = bubble(a1);
  console.timeEnd('js');

  console.time('rust');
  a2 = module.bubble(arr);
  console.timeEnd('rust');

  // console.log(a1, a2)

})


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

function bubble (nums) {
  for(let i = 0; i < nums.length; i++) {
    for(let j = 0; j < nums.length - 1; j++) {
      if (nums[j] > nums[j + 1]) {
        exchange(nums, j, j + 1);
      }
    }
  }

  return nums
}

function exchange(nums, i, j) {
  let temp = nums[i];
  nums[i] = nums[j];
  nums[j] = temp;
}

