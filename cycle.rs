use std::io;

fn main() {
  let mut s2 = String::new();
  io::stdin().read_line(&mut s2).expect("err");
  let mut s1 = String::new();
  io::stdin().read_line(&mut s1).expect("err");
  let c1: u16 = s1.trim().parse().expect("err");
  for i in 0..c1 {
    println!("{0}: {1}", i, s2.trim());
  }
}