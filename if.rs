use std::io;

fn main() {
  let mut s1 = String::new();
  io::stdin().read_line(&mut s1).expect("err");
  let c1: i64 = s1.trim().parse().expect("err");
  if ((c1 % 4 == 0 && c1 % 100 != 0) || (c1%400 == 0)) {
    println!("{} является високосным годом", c1);
  } else {
    println!("{} не является високосным годом", c1);
  }
}
