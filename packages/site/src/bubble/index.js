function exchange(nums, i, j) {
  let temp = nums[i];
  nums[i] = nums[j];
  nums[j] = temp;
}

export default function (input) {
  // 模拟克隆操作
  let nums = JSON.parse(JSON.stringify(input));
  for (let i = 0; i < nums.length; i++) {
    for (let j = 0; j < nums.length - 1; j++) {
      if (nums[j] > nums[j + 1]) {
        exchange(nums, j, j + 1);
      }
    }
  }

  return nums
}
