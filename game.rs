use std::io;

fn main() {
  let mut s1 = String::new();
  io::stdin().read_line(&mut s1).expect("err");
  let mut c1: u8 = s1.trim().parse().expect("err");
  let mut s2 = String::new();
  io::stdin().read_line(&mut s2).expect("err");
  let mut c2: u8 = s2.trim().parse().expect("err");
  let mut p: f64 = 0.0;
  let mut d: f64 = 0.0;
  for i in 1..=6 {
    for j in 1..=6 {
      if (i + j == c1) {
        p += 1.0;
      }
      if (i + j == c1 && (i == c2 || j == c2)) {
        d += 1.0;
      }
    }
  }
  println!("{:.2}", d/p);
}
