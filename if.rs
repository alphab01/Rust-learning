use std::io;

fn main() {
  let mut s1 = String::new();
  io::stdin().read_line(&mut s1).expect("err");
  let mut c: i64 = s1.trim().parse().expect("err");
  let mut ma: i64 = 0;
  let (mut c1, mut c2, mut c3) = (c/100, (c/10)%10, c%10);
  if (c1 * 100 + c2 * 10 + c3 > ma) {
    ma = c1 * 100 + c2 * 10 + c3;
  }
  if (c2 * 100 + c1 * 10 + c3 > ma) {
    ma = c2 * 100 + c1 * 10 + c3;
  }
  if (c3 * 100 + c2 * 10 + c1 > ma) {
    ma = c3 * 100 + c2 * 10 + c1;
  }
  if (c1 * 100 + c3 * 10 + c2 > ma) {
    ma = c1 * 100 + c3 * 10 + c2;
  }
  if (c3 * 100 + c1 * 10 + c2 > ma) {
    ma = c3 * 100 + c1 * 10 + c2;
  }
  if (c2 * 100 + c3 * 10 + c1 > ma) {
    ma = c2 * 100 + c3 * 10 + c1;
  }
  println!("{ma}");
}
