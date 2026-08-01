use std::io;

fn main() {
  let mut s1 = String::new();
  io::stdin().read_line(&mut s1).expect("err");
  let mut c1: u8 = s1.trim().parse().expect("err");
  let mut s2 = String::new();
  io::stdin().read_line(&mut s2).expect("err");
  let mut c2: u8 = s2.trim().parse().expect("err");
  let mut s3 = String::new();
  io::stdin().read_line(&mut s3).expect("err");
  let mut c3: u16 = s3.trim().parse().expect("err");
  if ((c3 % 4 == 0 && c3 % 100 != 0) || c3 % 400 == 0) {
    if (c2 == 1 && c1 > 0 && c1 < 32) {
      println!("Дата корректна!");
    } else if (c2 == 2 && c1 > 0 && c1 < 30) {
      println!("Дата корректна!");
    } else if (c2 == 3 && c1 > 0 && c1 < 32) {
      println!("Дата корректна!");
    } else if (c2 == 4 && c1 > 0 && c1 < 31) {
      println!("Дата корректна!");
    } else if (c2 == 5 && c1 > 0 && c1 < 32) {
      println!("Дата корректна!");
    } else if (c2 == 6 && c1 > 0 && c1 < 31) {
      println!("Дата корректна!");
    } else if (c2 == 7 && c1 > 0 && c1 < 32) {
      println!("Дата корректна!");
    } else if (c2 == 8 && c1 > 0 && c1 < 32) {
      println!("Дата корректна!");
    } else if (c2 == 9 && c1 > 0 && c1 < 31) {
      println!("Дата корректна!");
    } else if (c2 == 10 && c1 > 0 && c1 < 32) {
      println!("Дата корректна!");
    } else if (c2 == 11 && c1 > 0 && c1 < 31) {
      println!("Дата корректна!");
    } else if (c2 == 12 && c1 > 0 && c1 < 32) {
      println!("Дата корректна!");
    } else {
      println!("Дата некорректна!");
    }
  } else {
    if (c2 == 1 && c1 > 0 && c1 < 32) {
      println!("Дата корректна!");
    } else if (c2 == 2 && c1 > 0 && c1 < 29) {
      println!("Дата корректна!");
    } else if (c2 == 3 && c1 > 0 && c1 < 32) {
      println!("Дата корректна!");
    } else if (c2 == 4 && c1 > 0 && c1 < 31) {
      println!("Дата корректна!");
    } else if (c2 == 5 && c1 > 0 && c1 < 32) {
      println!("Дата корректна!");
    } else if (c2 == 6 && c1 > 0 && c1 < 31) {
      println!("Дата корректна!");
    } else if (c2 == 7 && c1 > 0 && c1 < 32) {
      println!("Дата корректна!");
    } else if (c2 == 8 && c1 > 0 && c1 < 32) {
      println!("Дата корректна!");
    } else if (c2 == 9 && c1 > 0 && c1 < 31) {
      println!("Дата корректна!");
    } else if (c2 == 10 && c1 > 0 && c1 < 32) {
      println!("Дата корректна!");
    } else if (c2 == 11 && c1 > 0 && c1 < 31) {
      println!("Дата корректна!");
    } else if (c2 == 12 && c1 > 0 && c1 < 32) {
      println!("Дата корректна!");
    } else {
      println!("Дата некорректна!");
    }
  }
}
