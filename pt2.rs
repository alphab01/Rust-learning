use std::io;

fn main() {
  let mut s1 = String::new();
  let mut s2 = String::new();
  let mut s3 = String::new();
  let mut s4 = String::new();
  io::stdin().read_line(&mut s1).expect("err");
  io::stdin().read_line(&mut s2).expect("err");
  io::stdin().read_line(&mut s3).expect("err");
  io::stdin().read_line(&mut s4).expect("err");
  let c1: f64 = s1.trim().parse().expect("err");
  let c2: f64 = s2.trim().parse().expect("err");
  let c3: f64 = s3.trim().parse().expect("err");
  let c4: f64 = s4.trim().parse().expect("err");
  let d: f64 = (2.0 * c4 - 2.0 * c1 * c3) / ((c3 - 1.0) * c3);
  println!("За {0} день турист прошел {1:.3} км", c2, c1 + (c2 - 1.0) * d);
}
