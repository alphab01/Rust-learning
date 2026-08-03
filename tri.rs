use std::io;

fn main() {
  let mut s = String::new();
  io::stdin().read_line(&mut s).expect("err");
  let mut c: usize = s.trim().parse().expect("err");
  for i in 1..=c {
    for j in 1..=i {
      if (j != i) {
        print!("{j} ");
      } else {
        println!("{j}");
      }
    } 
  }
}
