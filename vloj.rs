use std::io;

fn main() {
  let mut s = String::new();
  io::stdin().read_line(&mut s).expect("err");
  let mut c: u32 = s.trim().parse().expect("err");
  let mut s1 = String::new();
  io::stdin().read_line(&mut s1).expect("err");
  let mut c1: u32 = s1.trim().parse().expect("err");
  let mut i: u32 = 1;
  while i <= c {
    for j in 0..c1 {
      if (j != c1 - 1 && i != c) {
        print!("{i} ");
      } else {
        print!("{i}\n");
      }
      i += 1;
      if (i > c) {
        break;
      }
    }
  }
}
