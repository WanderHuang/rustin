pub fn run(arr: &mut Vec<i32>, start: usize, end: usize) -> &mut Vec<i32> {
  let mid = partition(arr, start, end);
  if mid > 0 && start <= mid - 1 {
    run(arr, start, mid - 1);
  }
  if mid + 1 <= end {
    run(arr, mid + 1, end);
  }

  arr
}

pub fn partition(arr: &mut Vec<i32>, start: usize, end: usize) -> usize {
  let pivot = arr[start];

  let mut start = start;
  let mut end = end;

  while start < end {
    while start < end && arr[end] > pivot {
      end -= 1;
    }
    if start < end {
      arr[start] = arr[end];
    }
    while start < end && arr[start] < pivot {
      start += 1;
    }
    if start < end {
      arr[end] = arr[start];
    }
  }
  arr[start] = pivot;
  start
}

#[cfg(test)]
mod tests {
  #[test]
  fn quick_sort_test() {
    let arr: [i32; 10] = [3, 1, 5, 2, 0, 7, 9, 4, 8, 6];
    let res = crate::quick_sort::run(&mut arr.to_vec(), 0, 9).to_vec();
    assert_eq!(res, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
  }
}
