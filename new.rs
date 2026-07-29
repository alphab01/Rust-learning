use std::io;

fn main() {
  let mut s = String::new();
  let mut b = String::new();
  io::stdin().read_line(&mut s).expect("err");
  io::stdin().read_line(&mut b).expect("err");
  let a: i32 = b.trim().parse().expect("err");
  let c: i32 = s.trim().parse().expect("err");
  let d = c % a;
  println!("На поле доступно еще {} кв.м свободного места", d);
  
}
