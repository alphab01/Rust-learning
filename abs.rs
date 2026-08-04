use std::io;

fn main() {
  let mut s1 = String::new();
  io::stdin().read_line(&mut s1).expect("err");
  let mut c1: f64 = s1.trim().parse().expect("err");
  calc_radians(c1);
}

fn calc_radians(mut degrees: f64) {
  let mut d: f64 = degrees;
  degrees = (degrees * 3.1415926535)/180.0;
  let mut c: f64 = 0.0;
  for i in 0..10 {
    c += cos(degrees, i);
  }
  let mut s: f64 = 0.0;
  for i in 0..10 {
    s += sin(degrees, i);
  }
  if (d != 180.0) {
    println!("sin({d:.1}) = {s:.9}\ncos({d:.1}) = {c:.9}");
  } else {
    println!("sin({d:.1}) = 0.000000000\ncos({d:.1}) = {c:.9}");
  }
}

fn pow(x: f64, n: u64) -> f64 {
  if (n == 0) {
    return 1.0;
  } else {
    return x * pow(x, n - 1);
  }
}

fn f(n: u64) -> f64{
  if (n == 0) {
    return 1.0;
  } else {
    return f(n - 1) * n as f64;
  }
}

fn sin(x: f64, n: u64) -> f64 {
  return (pow(-1.0, n) * pow(x, 2 * n + 1))/(f(2 * n + 1));
}

fn cos(x: f64, n: u64) -> f64 {
  return (pow(-1.0, n) * pow(x, 2 * n))/(f(2 * n));
}
