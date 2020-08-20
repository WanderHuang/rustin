pub fn run(arr: &mut [i32]) ->&mut [i32]  {
  let mut new_len: usize;
  let mut len = arr.len();
  loop {
    new_len = 0;
    for i in 1..len {
      if arr[i - 1] > arr[i] {
        arr.swap(i - 1, i);
        new_len = i;
      }
    }
    if new_len == 0 {
      break;
    }
    len = new_len;
  }

  arr
  // 另一种方式
  // let len = arr.len();
  // for _i in 0..len {
  //   for j in 0..len - 1 {
  //     if arr[j] > arr[j + 1] {
  //       arr.swap(j, j + 1);
  //     }
  //   }
  // }

  // arr
}

#[cfg(test)]
mod tests {
  use rand::seq::SliceRandom;
  use rand::thread_rng;
  use std::time::{SystemTime};

  fn shuffle(arr: &mut Vec<i32>) -> Vec<i32> {
    let mut rng = thread_rng();
    arr.shuffle(&mut rng);
    arr.to_vec()
  }
  #[test]
  fn bubble_test() {
    let len = 20000;
    let mut arr: Vec<i32> = Vec::new();
    let mut i = 0;
    loop {
      arr.push(i);
      i += 1;
      if  i == len {
        break;
      }
    }

    let mut input = shuffle(&mut arr);
    let before = SystemTime::now();

    println!("=========================== start ======================");

    crate::bubble::run(&mut input).to_vec();
    println!("duration {:?}", SystemTime::now().duration_since(before).unwrap());
  }
}