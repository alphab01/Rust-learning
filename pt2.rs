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
  println!("Концентрация получившегося раствора: {:.3}%", (c1*c2) / (c1 + c3));
}
