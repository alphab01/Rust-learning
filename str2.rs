use std::io;

fn main() {
  let mut str = String::new();
  io::stdin().read_line(&mut str);
  let a: i32 = str.trim().parse().expect("err\n");
  // сверху можно ещё добавить expect и в случае, если введены не только цифры, то прога аварийно заканчивается
  println!("{a}");
}
