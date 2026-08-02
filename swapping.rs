use std::io;

fn main() {
  let mut s1 = String::new();
  io::stdin().read_line(&mut s1).expect("err");
  let c1: usize = s1.trim().parse().expect("err");
  let mut arr: [i64; 10] = [0; 10];
  for i in 0..10 {
    let mut s2 = String::new();
    io::stdin().read_line(&mut s2).expect("err");
    let c2: i64 = s2.trim().parse().expect("err");
    arr[i] = c2;
  }
  arr[c1 - 1] ^= arr[c1 + 1];
  arr[c1 + 1] ^= arr[c1 - 1];
  arr[c1 - 1] ^= arr[c1 + 1];
  println!("{:?}", arr);
}
