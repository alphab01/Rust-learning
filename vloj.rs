use std::io;

fn main() {
  let mut s = String::new();
  io::stdin().read_line(&mut s).expect("err");
  let mut c: u8 = s.trim().parse().expect("err");
  for i in 0..c {
    for j in 0..(c - i - 1) {
      print!(" ");
    }
    for j in 0..(1 + i * 2) {
      print!("*");
    }
    println!("");
  }
}
