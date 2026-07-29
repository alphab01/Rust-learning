use std::io;

fn main() {
  let mut a = String::new();
  let mut b = String::new();
  let mut c = String::new();
  let mut d = String::new();
  io::stdin().read_line(&mut a);
  io::stdin().read_line(&mut b);
  io::stdin().read_line(&mut c);
  io::stdin().read_line(&mut d);
  println!("{0} {1} {2} {3}", d.trim(), c.trim(), b.trim(), a.trim());
}
