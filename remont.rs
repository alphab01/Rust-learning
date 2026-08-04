use std::io;

fn get_len_width() {
  let mut s1 = String::new();
  io::stdin().read_line(&mut s1).expect("err");
  let mut c1: f64 = s1.trim().parse().expect("err");
  let mut s2 = String::new();
  io::stdin().read_line(&mut s2).expect("err");
  let mut c2: f64 = s2.trim().parse().expect("err");
  let mut s3 = String::new();
  io::stdin().read_line(&mut s3).expect("err");
  let mut c3: f64 = s3.trim().parse().expect("err");
  calc_cost(c1, c2, c3);
}

fn main() {
  get_len_width();
}

fn calc_cost(length: f64, width: f64, price: f64) {
  print_cost(length * width * price);
}

fn print_cost(cost: f64) {
  println!("{cost:.2}");
}