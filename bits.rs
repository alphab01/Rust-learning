use std::io;

fn main() {
  let mut s1 = String::new();
  io::stdin().read_line(&mut s1).expect("err");
  let mut c1: u8 = s1.trim().parse().expect("err");
  let mut s2 = String::new();
  io::stdin().read_line(&mut s2).expect("err");
  let mut c2: u8 = s2.trim().parse().expect("err");
  let mut c3: u8 = c1 | c2;
  let mut c: u8 = 0;
  while (c3 > 0) {
    c += c3 % 2;
    c3 /= 2;
  }
  let mut b: u8 = 0;
  let mut i: u8 = 0;
  while (c1 > 0 && c2 > 0) {
    if (c1 % 2 != c2 % 2) {
      b += 1;
    }
    c1 /= 2;
    c2 /= 2;
  }
  while (c2 != 0) {
    i += 1;
    c2 /= 2;
  }
  while (c1 != 0) {
    i += 1;
    c1 /= 2;
  }
  println!("{}", b + i);
}
