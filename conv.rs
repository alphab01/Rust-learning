use std::io;

fn main() {
  let mut s = String::new();
  io::stdin().read_line(&mut s).expect("err");
  let c: i32 = s.trim().parse().expect("err");
  println!("{:#b}", c);
  println!("{:#o}", c);
  println!("{:#x}", c);   
}