(function (global) {
  function quickSortInner(arr, start, end) {
    let mid = partition(arr, start, end);
    if (mid > 0 && start <= mid - 1) {
      quickSortInner(arr, start, mid - 1);
    }
    if (mid + 1 <= end) {
      quickSortInner(arr, mid + 1, end);
    }

    return arr;
  }

  function partition(arr, start, end) {
    let pivot = arr[start];
    while (start < end) {
      while (start < end && arr[end] > pivot) end--;
      if (start < end) arr[start] = arr[end];
      while (start < end && arr[start] < pivot) start++;
      if (start < end) arr[end] = arr[start];
    }

    arr[start] = pivot;

    return start;
  }

  global.ev.register('js_quickSort', function quickSort(arr) {
    return quickSortInner(arr, 0, arr.length - 1);
  });
})(global);
