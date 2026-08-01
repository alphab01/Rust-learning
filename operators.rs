use std::io;

fn main() {
  let mut s = String::new();
  io::stdin().read_line(&mut s).expect("err");
  let d: i64 = s.trim().parse().expect("err");
  let mut arr = [1, 2, 3, 4, 5];
  arr[0] += d;
  arr[1] -= d;
  arr[2] *= d;
  arr[3] /= d;
  arr[4] %= d;
  println!("{:?}", arr);
}
