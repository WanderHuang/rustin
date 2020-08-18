pub fn run(arr: &mut Vec<i32>) -> &mut Vec<i32>  {
  let len = arr.len();
  for _i in 0..len {
    for j in 0..len - 1 {
      if arr[j] > arr[j + 1] {
        arr.swap(j, j + 1);
      }
    }
  }

  arr
}
