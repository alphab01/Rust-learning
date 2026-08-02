use std::io;

fn main() {
  let mut s1 = String::new();
  io::stdin().read_line(&mut s1).expect("err");
  let mut c1: i64 = s1.trim().parse().expect("err");
  let mut s2 = String::new();
  io::stdin().read_line(&mut s2).expect("err");
  let mut c2: u8 = s2.trim().parse().expect("err");
  let mut c: i64 = 0;
  for i in 0..c2 {
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("err");
    let xD: i64 = s.trim().parse().expect("err");
    if (xD > c1) {
      c += 1;
    }
  }
  println!("Количество элементов больших {c1}: {c}");
}

