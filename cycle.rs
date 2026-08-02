use std::io;

fn main() {
  let mut s1 = String::new();
  io::stdin().read_line(&mut s1).expect("err");
  let c1: i16 = s1.trim().parse().expect("err");
  let mut s2 = String::new();
  io::stdin().read_line(&mut s2).expect("err");
  let c2: i16 = s2.trim().parse().expect("err");
  for i in 1..=10 {
    println!("{0} x {1} = {2}", c1, i, c1 * i);
  }
  println!("");
  for i in 1..=10 {
    println!("{0} x {1} = {2}", c2, i, c2 * i);
  } 
}

/*
Если кто-то когда-то увидит этот код и время его отправки, то я надеюсь вас не смутит актуальность
*/