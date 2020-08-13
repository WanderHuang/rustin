pub fn run(arr: &[f64]) -> Vec<f64> {
  let len = arr.len();
  let mut copy = arr.to_vec();
  for _i in 0..len {
    for j in 0..len - 1 {
      if copy[j] > copy[j + 1] {
        copy.swap(j, j + 1);
      }
    }
  }

  copy
}
