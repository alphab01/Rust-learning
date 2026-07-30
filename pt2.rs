use std::io;

fn main() {
  let mut s1 = String::new();
  let mut s2 = String::new();
  io::stdin().read_line(&mut s1).expect("err");
  io::stdin().read_line(&mut s2).expect("err");
  let c1: f64 = s1.trim().parse().expect("err");
  let c2: f64 = s2.trim().parse().expect("err");
  println!("{:.3}", c1 * (c2 / 100.0));
}
