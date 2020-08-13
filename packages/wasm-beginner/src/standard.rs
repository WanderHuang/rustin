pub fn run(arr: &[f64]) -> Vec<f64> {
  let mut copy = arr.to_vec();
  copy.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
  copy
}