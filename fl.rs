use std::io;

fn main() {
  let mut s = String::new();
  io::stdin().read_line(&mut s).expect("err");
  let c: f64 = s.trim().parse().expect("err");
  let d = c as i32;
  println!("{0}\n{1:.3}", d, c - d as f64);
}