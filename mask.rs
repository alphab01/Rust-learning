use std::io;

fn main() {
  let mut s1 = String::new();
  io::stdin().read_line(&mut s1).expect("err");
  let c1: i8 = s1.trim().parse().expect("err");
  let mut s2 = String::new();
  io::stdin().read_line(&mut s2).expect("err");
  let c2: i8 = s2.trim().parse().expect("err");
  if (c1 & (1 << 7) == c2 & (1 << 7)) {
    println!("числа {0} и {1} имеют одинаковые знаки", c1, c2);
  } else {
    println!("числа {0} и {1} имеют разные знаки", c1, c2);
  }
}
