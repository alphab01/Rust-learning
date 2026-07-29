use std::io;

fn main() {
  let arr = [-1, 0, 1, 2, 30, 4, 500];
  let mut s = String::new();
  io::stdin().read_line(&mut s).expect("err");
  let d: usize = s.trim().parse().expect("err");
  println!("{}", arr[d]);
}
