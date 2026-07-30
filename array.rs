use std::io;

fn main() {
  let mut s1 = String::new();
  let mut s2 = String::new();
  let mut s3 = String::new();
  let mut s4 = String::new();
  let mut s5 = String::new();
  let mut arr: [usize; 5] = [0; 5];
  io::stdin().read_line(&mut s1).expect("err");
  io::stdin().read_line(&mut s2).expect("err");
  io::stdin().read_line(&mut s3).expect("err");
  io::stdin().read_line(&mut s4).expect("err");
  io::stdin().read_line(&mut s5).expect("err");
  arr[0] = s1.trim().parse().expect("err");
  arr[1] = s2.trim().parse().expect("err");
  arr[2] = s3.trim().parse().expect("err");
  arr[3] = s4.trim().parse().expect("err");
  arr[4] = s5.trim().parse().expect("err");
  println!("{0}, {1}, {2}, {3}, {4}", arr[arr[0]], arr[arr[1]], arr[arr[2]], arr[arr[3]], arr[arr[4]]);
}
