use std::io;

fn main() {
  let mut a = String::new();
  let mut b = String::new();
  io::stdin().read_line(&mut a).expect("err");
  io::stdin().read_line(&mut b).expect("err");
  let a1: i32 = a.trim().parse().expect("err");
  let b1: i32 = b.trim().parse().expect("err");
  let c = a1 + b1;
  println!("{}", c);
}
