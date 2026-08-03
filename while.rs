use std::io;

fn main() {
  let mut s1 = String::new();
  io::stdin().read_line(&mut s1).expect("err");
  let mut s: f64 = 0.0;
  while true {
    let mut s2 = String::new();
    io::stdin().read_line(&mut s2).expect("err");
    if (s1.trim() == s2.trim()) {
      break;
    } else {
      let c: f64 = match s2.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,
      };
      s += c;
    }
  }
  println!("{s:.1}");
}
