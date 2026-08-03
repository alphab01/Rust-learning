use std::io;

fn main() {
  let mut s1 = String::new();
  io::stdin().read_line(&mut s1).expect("err");
  let mut c1: u32 = s1.trim().parse().expect("err");
  let mut s2 = String::new();
  io::stdin().read_line(&mut s2).expect("err");
  let mut c2: u32 = s2.trim().parse().expect("err");
  let mut c: u32 = c1;
  let mut i: u32 = 0;
  while (c > 0) {
    i += 1;
    c /= 10;
  }
  if (i <= c2) {
    println!("k >= n");
  } else {
    println!("{}", c1 % (10_u32.pow(i - c2)));
  }
}