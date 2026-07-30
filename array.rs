use std::io;

fn main() {
  let mut arr = [1.1, 1.1, 1.1, 1.1, 1.1];
  let mut s1 = String::new();
  let mut s2 = String::new();
  let mut s3 = String::new();
  let mut s4 = String::new();
  let mut s5 = String::new();
  let mut s6 = String::new();
  io::stdin().read_line(&mut s1).expect("err");
  io::stdin().read_line(&mut s2).expect("err");
  io::stdin().read_line(&mut s3).expect("err");
  io::stdin().read_line(&mut s4).expect("err");
  io::stdin().read_line(&mut s5).expect("err");
  io::stdin().read_line(&mut s6).expect("err");
  let d1: f64 = s1.trim().parse().expect("err");
  let d2: f64 = s2.trim().parse().expect("err");
  let d3: f64 = s3.trim().parse().expect("err");
  let d4: f64 = s4.trim().parse().expect("err");
  let d5: f64 = s5.trim().parse().expect("err");
  let d6: usize = s6.trim().parse().expect("err");
  arr[0] = d1;
  arr[1] = d2;
  arr[2] = d3;
  arr[3] = d4;
  arr[4] = d5;
  println!("{:.2}", arr[d6]);
}
