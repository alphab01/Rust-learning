use std::io;

fn main() {
  let mut s2 = String::new();
  io::stdin().read_line(&mut s2).expect("err");
  let mut s1 = String::new();
  io::stdin().read_line(&mut s1).expect("err");
  let c1: i32 = s1.trim().parse().expect("err");
  let mut s3 = String::new();
  io::stdin().read_line(&mut s3).expect("err");
  let c3: i32 = s3.trim().parse().expect("err");
  for i in c1..c3{
    println!("{}", s2.trim());
  }
}