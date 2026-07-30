use std::io;

fn main() {
  let mut arr = [-621.5, 11.1, 2.0, -7.123, 0.125, 0.0, 0.000051789];
  let mut s1 = String::new();
  let mut s2 = String::new();
  io::stdin().read_line(&mut s1).expect("err");
  io::stdin().read_line(&mut s2).expect("err");
  let c1: usize = s1.trim().parse().expect("err");
  let c2: usize = s2.trim().parse().expect("err");
  (arr[c1], arr[c2]) = (arr[c2], arr[c1]);
  println!("{:.9?}", arr);
}
