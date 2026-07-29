use std::io;

fn main() {
  let mut s = String::new();
  io::stdin().read_line(&mut s).expect("err");
  let c: f64 = s.trim().parse().expect("err");
  let mut s2 = String::new();
  io::stdin().read_line(&mut s2).expect("err");
  let b: f64 = s2.trim().parse().expect("err");
  println!("{0} kg = {1:.3} lbs", c, c * 2.205);
  println!("{0} lbs = {1:.3} kg", b, b * 0.454);
}