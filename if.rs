use std::io;

fn main() {
  let mut s = String::new();
  io::stdin().read_line(&mut s).expect("err");
  let c: f64 = s.trim().parse().expect("err");
  if (c > 0.0) {
    println!("Число {:.1} является положительным", c);
  } else {
    println!("Число {:.1} является отрицательным", c);
  }
}
