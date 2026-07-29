use std::io;

fn main() {
  let mut s = String::new();
  io::stdin().read_line(&mut s).expect("err");
  let c: f64 = s.trim().parse().expect("err");
  println!("{:E}", c);
}
