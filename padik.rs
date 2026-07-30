use std::io;

fn main() {
  let mut s1 = String::new();
  let mut s2 = String::new();
  let mut s3 = String::new();
  io::stdin().read_line(&mut s1).expect("err");
  io::stdin().read_line(&mut s2).expect("err");
  io::stdin().read_line(&mut s3).expect("err");
  let c1: f64 = s1.trim().parse().expect("err");
  let c2: f64 = s2.trim().parse().expect("err");
  let c3: f64 = s3.trim().parse().expect("err");
  println!("Максимальная глубина кодирования: {:.0}", (c2 * 8.0 * 1024.0)/(2.0 * 1000.0 * c1 * (1.0 - (c3/100.0))));
}
