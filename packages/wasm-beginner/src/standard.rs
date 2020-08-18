pub fn run(arr: &mut [i32]) -> Vec<i32> {
  let mut copy = arr.to_vec();
  copy.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
  copy
}