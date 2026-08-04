use std::io;

fn ipow(b: f64, st: i32) -> f64 {
  if (st == 0) {
    return 1.0;
  } else if (st < 0) {
    return 1.0/(ipow(b, -st));
  } else if (st > 0 && st % 2 == 0) {
    return ipow(b, st/2) * ipow(b, st/2);
  } else {
    return b * ipow(b, (st - 1)/2) * ipow(b, (st - 1)/2);
  }
}

fn main() {
  let mut s1 = String::new();
  io::stdin().read_line(&mut s1).expect("err");
  let mut c1: f64 = s1.trim().parse().expect("err");
  let mut s2 = String::new();
  io::stdin().read_line(&mut s2).expect("err");
  let mut c2: i32 = s2.trim().parse().expect("err");
  println!("{:.3}", ipow(c1, c2));
}