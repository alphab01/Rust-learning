use std::io;

fn main() {
  let mut s1 = String::new();
  io::stdin().read_line(&mut s1).expect("err");
  let mut c1: u32 = s1.trim().parse().expect("err");
  let xd: usize = c1 as usize;
  for i in 0..2_u32.pow(c1) {
    print!("{:0xd$b} | {1}\n", i, if (i != 2_u32.pow(c1) - 1) {0} else {1});
  }
}

