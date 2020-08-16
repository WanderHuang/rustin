(function (global) {
  global.ev.register('js_standard', function (nums) {
    nums.sort((a, b) => a - b);
    return nums;
  });
})(global);
