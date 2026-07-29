use std::io;

fn main() {
  let mut s = String::new();
  let mut b = String::new();
  io::stdin().read_line(&mut s).expect("err");
  io::stdin().read_line(&mut b).expect("err");
  let a: f64 = b.trim().parse().expect("err");
  let c: f64 = s.trim().parse().expect("err");
  println!("{0} + ({1}) = {2}\n{0} - ({1}) = {3}\n{0} * ({1}) = {4}\n{0} / ({1}) = {5:.3}\n{0} % ({1}) = {6:.3}\n", c, a, c + a, c - a, c * a, c / a, c % a);
}
