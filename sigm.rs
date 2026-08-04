use std::io;

fn main() {
  let mut s1 = String::new();
  io::stdin().read_line(&mut s1).expect("err");
  let mut c1: i64 = s1.trim().parse().expect("err");
  let mut s2 = String::new();
  io::stdin().read_line(&mut s2).expect("err");
  let mut c2: f64 = s2.trim().parse().expect("err");
  let mut s: f64 = 1.0;
  for i in 0..=c1 {
    s *= (c2 + 1.0)/(((c2 * c2)/2.0) + 4.0);
  }
  println!("{:.5}", s);
}
