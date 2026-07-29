use std::io;

fn main() {
  let mut s1 = String::new();
  let mut s2 = String::new();
  let mut s3 = String::new();
  let mut s4 = String::new();
  let mut s5 = String::new();
  io::stdin().read_line(&mut s1).expect("err");
  io::stdin().read_line(&mut s2).expect("err");
  io::stdin().read_line(&mut s3).expect("err");
  io::stdin().read_line(&mut s4).expect("err");
  io::stdin().read_line(&mut s5).expect("err");
  let a1: f64 = s1.trim().parse().expect("err");
  let a2: f64 = s2.trim().parse().expect("err");
  let a3: f64 = s3.trim().parse().expect("err");
  let a4: f64 = s4.trim().parse().expect("err");
  let a5: f64 = s5.trim().parse().expect("err");
  let tup = (a1, a2, a3, a4, a5);
  println!("{0}, {1}, {2}, {3}, {4}, 0", tup.0 as i64, tup.1 as i64, tup.2 as i64, tup.3 as i64, tup.4 as i64);
}