use std::io;

fn main() {
  let mut s1 = String::new();
  io::stdin().read_line(&mut s1).expect("err");
  let c1: u8 = s1.trim().parse().expect("err");
  let mut s2 = String::new();
  io::stdin().read_line(&mut s2).expect("err");
  let c2: u8 = s2.trim().parse().expect("err");
  println!("{0}-й бит числа {1:08b} равен {2}", c2, c1, if (c1 & (1 << c2) > 0) {1} else {0});
}
