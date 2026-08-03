use std::io;

fn main() {
  let mut s1 = String::new();
  io::stdin().read_line(&mut s1).expect("err");
  let mut c1: u32 = s1.trim().parse().expect("err");
  let mut c: u32 = 0;
  let mut c2: u32 = c1;
  while (c1 > 0) {
    c += 1;
    c1 /= 10;
  }
  let (mut i1, mut i2) = (c - 1, 0);
  let mut c3: bool = true;
  while (i1 > i2) {
    if ((c2 / (10_u32.pow(i1)))%10 != (c2 / (10_u32.pow(i2)))%10) {
      c3 = false;
    }
    i1 -= 1;
    i2 += 1;
  }
  if (c3) {
    println!("Число {} является палиндромом", c2);
  } else {
    println!("Число {} не является палиндромом", c2);
  }
}
