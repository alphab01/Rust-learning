use std::io;

fn main() {
  let mut s = String::new();
  io::stdin().read_line(&mut s).expect("err");
  let c: f64 = s.trim().parse().expect("err");
  let mut s2 = String::new();
  io::stdin().read_line(&mut s2).expect("err");
  let d: usize = s2.trim().parse().expect("err");
  println!("{0:.1$}", c, d);
}