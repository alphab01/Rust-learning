use std::io;

fn main() {
  let mut s = String::new();
  io::stdin().read_line(&mut s).expect("err");
  let a:i32 = s.trim().parse().expect("err");
  let mut s2 = String::new();
  io::stdin().read_line(&mut s2).expect("err");
  let b:i32 = s2.trim().parse().expect("err");
  println!("{0:#b} + {1:#b} = {2:#b}", a, b, a + b);
  println!("{0:#o} + {1:#o} = {2:#o}", a, b, a + b);
  println!("{0:#x} + {1:#x} = {2:#x}\n", a, b, a + b);
  println!("{0:#b} - {1:#b} = {2:#b}", a, b, a - b);
  println!("{0:#o} - {1:#o} = {2:#o}", a, b, a - b);
  println!("{0:#x} - {1:#x} = {2:#x}\n", a, b, a - b);
  println!("{0:#b} * {1:#b} = {2:#b}", a, b, a * b);
  println!("{0:#o} * {1:#o} = {2:#o}", a, b, a * b);
  println!("{0:#x} * {1:#x} = {2:#x}\n", a, b, a * b);
  println!("{0:#b} / {1:#b} = {2:#b}", a, b, a / b);
  println!("{0:#o} / {1:#o} = {2:#o}", a, b, a / b);
  println!("{0:#x} / {1:#x} = {2:#x}\n", a, b, a / b);
  println!("{0:#b} % {1:#b} = {2:#b}", a, b, a % b);
  println!("{0:#o} % {1:#o} = {2:#o}", a, b, a % b);
  println!("{0:#x} % {1:#x} = {2:#x}\n", a, b, a % b);
}
