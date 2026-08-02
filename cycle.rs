use std::io;

fn main() {
  let mut s1 = String::new();
  io::stdin().read_line(&mut s1).expect("err");
  let c1: u8 = s1.trim().parse().expect("err");
  let (mut ma, mut mi) = (0, 0);
  for i in 0..c1 {
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("err");
    let c: i32 = s.trim().parse().expect("err");
    if (i == 0) {
      (ma, mi) = (c, c);
    } else {
      if (ma < c) {
        ma = c;
      }
      if (mi > c) {
        mi = c;
      }
    }
  }
  println!("Максимальное число: {0}\nМинимальное число: {1}", ma, mi);
}