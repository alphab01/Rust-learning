use std::io;

fn main() {
  let mut str = String::new();

  let res = io::stdin().read_line(&mut str).expect("err\n");

  println!("{:#?}", str);
  println!("{res}");
}
