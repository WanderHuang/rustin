function exchange(nums, i, j) {
  let temp = nums[i];
  nums[i] = nums[j];
  nums[j] = temp;
}

export default function (input) {
  // 模拟克隆操作
  let nums = JSON.parse(JSON.stringify(input));
  let len = nums.length;
  let i = 0;
  while(i < len) {
    let j = 0;
    while(j < len - 1) {
      if (nums[j] > nums[j + 1]) {
        exchange(nums, j, j + 1);
      }
      j++
    }
    i++
  }

  return nums
}
