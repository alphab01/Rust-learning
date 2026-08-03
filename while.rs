use std::io;

fn main() {
  let mut s1 = String::new();
  io::stdin().read_line(&mut s1).expect("err");
  let mut c1: i64 = s1.trim().parse().expect("err");
  print!("Число {} состоит из цифр: ", c1);
  let mut c: i64 = 0;
  let mut c2: i64 = c1;
  while (c1 != 0) {
    c1 /= 10;
    c += 1;
  }
  let mut i: i64 = 1;
  while (i <= c) {
    if (i != c) {
      print!("{} ", (c2 / (10_u32.pow((c - i) as u32)) as i64)%10 as i64);
      i += 1;
    } else {
      print!("{}\n", (c2 / (10_u32.pow((c - i) as u32)) as i64)%10 as i64);
      i += 1;
    }
  }
  println!("Число {0} является {1} значным", c2, c);
}
