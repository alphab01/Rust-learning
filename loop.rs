use std::io;

fn main() {
  let mut s1 = String::new();
  io::stdin().read_line(&mut s1).expect("err");
  let mut s2 = String::new();
  io::stdin().read_line(&mut s2).expect("err");
  let mut c: u8 = s2.trim().parse().expect("err");
  let mut c2: u8 = 0;
  loop {
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("err");
    if (s == s1) {
      println!("Доступ предоставлен");
      break;
    } else {
      c2 += 1;
      println!("Неверный пароль");
      if (c2 >= c) {
        println!("Слишком много попыток, пожалуйста повторите позже");
        break;
      }
    }
  }
}