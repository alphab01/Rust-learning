use std::io;

fn main() {
  let mut s1 = String::new();
  io::stdin().read_line(&mut s1).expect("err");
  let mut c1: i64 = s1.trim().parse().expect("err");
  let mut s: f64 = 0.0;
  for i in 1..=c1 {
    if (i%2 != 0) {
      s += 1.0/(i as f64);
    } else {
      s -= 1.0/(i as f64);
    }
  }
  let mut c: usize = c1 as usize;
  println!("{:.c$}", s);
}
