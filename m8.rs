use std::io;

fn main() {
  let mut s1 = String::new();
  io::stdin().read_line(&mut s1).expect("err");
  let mut c1: i64 = s1.trim().parse().expect("err");
  let mut s2 = String::new();
  io::stdin().read_line(&mut s2).expect("err");
  let mut c2: i64 = s2.trim().parse().expect("err");
  let mut s3 = String::new();
  io::stdin().read_line(&mut s3).expect("err");
  let mut c3: i64 = s3.trim().parse().expect("err");
  let mut s4 = String::new();
  io::stdin().read_line(&mut s4).expect("err");
  let mut c4: i64 = s4.trim().parse().expect("err");
  let mut s5 = String::new();
  io::stdin().read_line(&mut s5).expect("err");
  let mut c5: i64 = s5.trim().parse().expect("err");
  let mut s6 = String::new();
  io::stdin().read_line(&mut s6).expect("err");
  let mut c6: i64 = s6.trim().parse().expect("err");
  if ((c3 - c1) * (c6 - c2) - (c5 - c1) * (c4 - c2) == 0) {
    println!("Точки коллинеарны");
  } else {
    println!("Точки не коллинеарны");
  }
}
