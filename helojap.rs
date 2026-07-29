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
  println!("{0}{1}{2}{3}{4}", s1.trim(), s2.trim(), s3.trim(), s4.trim(), s5.trim())
}
