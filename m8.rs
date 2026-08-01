use std::io;

fn main() {
  let mut s1 = String::new();
  io::stdin().read_line(&mut s1).expect("err");
  let mut c1: i64 = s1.trim().parse().expect("err");
  if (c1 == 1 || c1 == 2 || c1 == 4) {
    println!("Для {0}! последняя цифра равна {0}", c1);
  } else if (c1 == 3) {
    println!("Для 3! последняя цифра равна 6");
  } else {
    println!("Для {}! последняя цифра равна 0", c1);
  }
}
/*
1 1
2 2
3 6
4 4
5 0

*/