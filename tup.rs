use std::io;

fn main() {
  let mut s1 = String::new();
  let mut s2 = String::new();
  let mut s3 = String::new();
  let mut s4 = String::new();
  let mut s5 = String::new();
  io::stdin().read_line(&mut s1).expect("err");
  io::stdin().read_line(&mut s2).expect("err");
  io::stdin().read_line(&mut s3).expect("err");
  io::stdin().read_line(&mut s4).expect("err");
  io::stdin().read_line(&mut s5).expect("err");
  let tup = (s1, s2, s3, s4, s5);
  println!("{:?}", tup);
}