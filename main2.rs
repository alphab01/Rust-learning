use std::io;

fn main() {
  let mut s = String::new();
  io::stdin().read_line(&mut s).expect("err");
  let c: u64 = s.trim().parse().expect("err");
  println!("{0} сек = {1} час {2} минут {3} секунд", c, c / 3600, (c % 3600) / 60, ((c % 3600) % 60));
}
