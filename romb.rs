use std::io;

fn main() {
  let mut s1 = String::new();
  io::stdin().read_line(&mut s1).expect("err");
  let mut c1: i64 = s1.trim().parse().expect("err");
  let mut s2 = String::new();
  io::stdin().read_line(&mut s2).expect("err");
  let mut c2: i64 = s2.trim().parse().expect("err");
  let mut ma: i64 = -10000000;
  if (c1 == c2) {
    println!("Решения в целых числах нет");
  } else if (c1 > c2) {
    println!("A и B не пересекаются");
  } else {
    for i in c1..=c2 {
      if ((!(i <= c1)) && (i < c2)) {
        ma = i;
      }
    }
    println!("{ma}");
  }
}