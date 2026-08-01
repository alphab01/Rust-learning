use std::io;

fn main() {
  let mut s1 = String::new();
  io::stdin().read_line(&mut s1).expect("err");
  let c1: i8 = s1.trim().parse().expect("err");
  let mut s2 = String::new();
  io::stdin().read_line(&mut s2).expect("err");
  let c2: u8 = s2.trim().parse().expect("err");
  println!("x до очистки {0}-го бита\nв двоичной записи: {1:08b}\nв десятичной записи: {1}\n\nx после очистки {0}-го бита\nв двоичной записи: {2:08b}\nв десятичной записи: {2}", c2, c1, c1 & (!(1 << c2)));
}
