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
  if (c1 == c2 && c2 == c3) {
    println!("Числа {0}, {1} и {2} образуют равносторонний треугольник", c1, c2, c3);
  } else if ((c1 == c2 && c2 != c3) || (c1 == c3 && c2 != c3) || (c2 == c3 && c1 != c3)) {
    println!("Числа {0}, {1} и {2} образуют равнобедренный треугольник", c1, c2, c3);
  } else {
    if ((c1 + c2 > c3) && (c1 + c3 > c2) && (c2 + c3 > c1)) {
      println!("Числа {0}, {1} и {2} образуют разносторонний треугольник", c1, c2, c3);
    } else {
      println!("Числа {c1}, {c2} и {c3} не образуют треугольник");
    }
  }
}
