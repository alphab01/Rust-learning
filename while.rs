use std::io;

fn main() {
  let mut s1 = String::new();
  io::stdin().read_line(&mut s1).expect("err");
  let mut c1: u8 = s1.trim().parse().expect("err");
  let mut c: u8 = 0;
  let s: u8 = c1;
  while (c1 != 0) {
    c += c1 % 2;
    c1 /= 2;
  }
  println!("Количество установленных битов в {0:08b} равно {1}", s, c);
}
