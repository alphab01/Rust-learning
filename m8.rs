use std::io;

fn main() {
  let mut s2 = String::new();
  io::stdin().read_line(&mut s2).expect("err");
  let mut s1 = String::new();
  io::stdin().read_line(&mut s1).expect("err");
  let mut c1: u8 = s1.trim().parse().expect("err");
  if (s2.trim() == "Север".to_string()) {
    if (c1 == 1) {
      s2 = "Запад".to_string();
    } else if (c1 == 2) {
      s2 = "Восток".to_string();
    }
  } else if (s2.trim() == "Запад".to_string()) {
    if (c1 == 1) {
      s2 = "Юг".to_string();
    } else if (c1 == 2) {
      s2 = "Север".to_string();
    }
  } else if (s2.trim() == "Юг".to_string()) {
    if (c1 == 1) {
      s2 = "Восток".to_string();
    } else if (c1 == 2) {
      s2 = "Запад".to_string();
    }
  } else if (s2.trim() == "Восток".to_string()) {
    if (c1 == 1) {
      s2 = "Север".to_string();
    } else if (c1 == 2) {
      s2 = "Юг".to_string();
    }
  }
  println!("Направление лунохода после выполнения команды: {}", s2);
}
