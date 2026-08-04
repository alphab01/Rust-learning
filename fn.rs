use std::io;

fn main() {
  let mut s = String::new();
  io::stdin().read_line(&mut s).expect("err");
  let mut c: usize = s.trim().parse().expect("err");
  get_primary_sc(c);
}

fn get_primary_sc(c: usize) {
  let a = [0, 7, 14, 20, 27, 34, 40, 43, 46, 48, 51, 54, 56, 59, 62, 64, 67, 70, 72, 75, 78, 80, 83, 85, 88, 90, 93, 95, 98, 100];
  print_test_sc(a[c]);
}

fn print_test_sc(scores: u8) {
  println!("{scores}");
}