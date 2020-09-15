use the_super_tiny_compiler::compile;
fn main() {
  let res = compile("add (123 678 789)");
  println!("===== Input  =====");
  println!("add (123 678 789)");
  println!("===== Result =====");
  println!("{}", res);
}

