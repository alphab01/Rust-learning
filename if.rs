use std::io;

fn main() {
  let mut s = String::new();
  io::stdin().read_line(&mut s).expect("err");
  let c: i64 = s.trim().parse().expect("err");
  let mut s1 = String::new();
  io::stdin().read_line(&mut s1).expect("err");
  let c1: i64 = s1.trim().parse().expect("err");
  if (c%2 == 0) {
    println!("Число {} является четным", c);
  } else {
    println!("Число {} является нечетным", c);
  }
  if (c1%2 == 0) {
    println!("Число {} является четным", c1);
  } else {
    println!("Число {} является нечетным", c1);
  }
}
