use std::io;

fn main() {
  let arr = [-2.5, 4.2, 9.1, 22.5, 30.0, 1445.123, 1000000.0, 0.001, 0.5, -0.127];
  let mut s = String::new();
  io::stdin().read_line(&mut s).expect("err");
  let c: i64 = s.trim().parse().expect("err");
  if c < 0 {
    println!("Отрицательный индекс приводит к панике");
  } else {
    if (c > 9) {
      println!("Попытка выхода за пределы массива");
    } else {
      println!("Элемент с индексом {0} равен {1:.3}", c, arr[c as usize]);
    }
  }
}