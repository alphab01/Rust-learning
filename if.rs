use std::io;

fn main() {
  let mut s1 = String::new();
  io::stdin().read_line(&mut s1).expect("err");
  let mut c1: f64 = s1.trim().parse().expect("err");
  let mut s2 = String::new();
  io::stdin().read_line(&mut s2).expect("err");
  let mut c2: f64 = s2.trim().parse().expect("err");
  let mut s3 = String::new();
  io::stdin().read_line(&mut s3).expect("err");
  let mut c3: f64 = s3.trim().parse().expect("err");
  if c1 > c2 && c1 > c3 && c2 > c3 {
    println!("{0:.1}, {1:.1}, {2:.1}", c3, c2, c1);
  }
  if c1 > c2 && c1 < c3 && c2 < c3 {
    println!("{0:.1}, {1:.1}, {2:.1}", c2, c1, c3);
  }
  if c1 > c2 && c1 > c3 && c2 < c3 {
    println!("{0:.1}, {1:.1}, {2:.1}", c2, c3, c1);
  }
  if c1 < c2 && c1 > c3 && c2 > c3 {
    println!("{0:.1}, {1:.1}, {2:.1}", c3, c1, c2);
  }
  if c1 < c2 && c1 < c3 && c2 > c3 {
    println!("{0:.1}, {1:.1}, {2:.1}", c1, c3, c2);
  }
  if c1 < c2 && c1 < c3 && c2 < c3 {
    println!("{0:.1}, {1:.1}, {2:.1}", c1, c2, c3);
  }
}
