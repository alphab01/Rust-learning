use std::io;

fn print_str() {
  println!("была вызвана функция!");
}

fn main() {
  let mut s = String::new();
  io::stdin().read_line(&mut s).expect("err");
  let mut c: usize = s.trim().parse().expect("err");
  for i in 1..=c {
    print!("{i}) ");
    print_str();
  }
}