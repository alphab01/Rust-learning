use std::io;

fn main() {
  let mut s1 = String::new();
  io::stdin().read_line(&mut s1).expect("err");
  let c1: i32 = s1.trim().parse().expect("err");
  let mut s2 = String::new();
  io::stdin().read_line(&mut s2).expect("err");
  let c2: i32 = s2.trim().parse().expect("err");
  for i in (c1..=c2).rev() {
    println!("{i}");
  }
}