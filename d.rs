use std::io;

fn main() {
  let mut s = String::new();
  io::stdin().read_line(&mut s).expect("err");
  let mut a: i64 = s.trim().parse().expect("err");
  let mut s2 = String::new();
  io::stdin().read_line(&mut s2).expect("err");
  let mut c: i64 = s2.trim().parse().expect("err");
  a = a - c;
  c = c + a;
  a = c - a;
  println!("{0}\n{1}", a, c);
}
