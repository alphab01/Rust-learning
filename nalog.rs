use std::io;

fn input() {
  let mut s1 = String::new();
  io::stdin().read_line(&mut s1).expect("err");
  let mut c1: f64 = s1.trim().parse().expect("err");
  let mut s2 = String::new();
  io::stdin().read_line(&mut s2).expect("err");
  let mut c2: f64 = s2.trim().parse().expect("err");
  let mut s3 = String::new();
  io::stdin().read_line(&mut s3).expect("err");
  let mut c3: f64 = s3.trim().parse().expect("err");
  calc_tax(c1, c2, c3);
}

fn main() {
    input();
}

fn calc_tax(c1: f64, c2: f64, c3: f64) {
  print_tax((c3 / 12.0) * c1 * c2);
}

fn print_tax(tax: f64) {
  println!("{tax:.2}");
}