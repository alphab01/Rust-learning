use std::io;

fn main() {
  let mut arr: [i64; 10] = [0; 10];
  for i in 0..10 {
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("err");
    let mut c: i64 = s.trim().parse().expect("err");
    arr[9 - i] = c;
  }
  for i in 0..10 {
    for j in 0..9 {
      if (arr[j] > arr[j + 1]) {
        arr[j] ^= arr[j + 1];
        arr[j + 1] ^= arr[j];
        arr[j] ^= arr[j + 1];
      }
    }
  }
  println!("{:?}", arr);
}
