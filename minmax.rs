use std::io;

fn main() {
  let array = [3, 1, 0, -5, -1, 300, 4, 2];
  let mut ma: i64 = array[0];
  if (ma < array[1]) {
    ma = array[1];
  }
  if (ma < array[2]) {
    ma = array[2];
  }
  if (ma < array[3]) {
    ma = array[3];
  }
  if (ma < array[4]) {
    ma = array[4];
  }
  if (ma < array[5]) {
    ma = array[5];
  }
  if (ma < array[6]) {
    ma = array[6];
  }
  if (ma < array[7]) {
    ma = array[7];
  }
  let mut mi: i64 = array[0];
  if (mi > array[1]) {
    mi = array[1];
  }
  if (mi > array[2]) {
    mi = array[2];
  }
  if (mi > array[3]) {
    mi = array[3];
  }
  if (mi > array[4]) {
    mi = array[4];
  }
  if (mi > array[5]) {
    mi = array[5];
  }
  if (mi > array[6]) {
    mi = array[6];
  }
  if (mi > array[7]) {
    mi = array[7];
  }
  let mut s1 = String::new();
  io::stdin().read_line(&mut s1).expect("err");
  let c1: usize = s1.trim().parse().expect("err");
  let mut s2 = String::new();
  io::stdin().read_line(&mut s2).expect("err");
  let c2: usize = s2.trim().parse().expect("err");
  if (array[c1] == mi) {
    println!("Считанный мин.индекс корректный");
  } else {
    println!("Считанный мин.индекс некорректный")
  }
  if (array[c2] == ma) {
    println!("Считанный макс.индекс корректный");
  } else {
    println!("Считанный макс.индекс некорректный")
  }
}
