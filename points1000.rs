use std::io;

fn main() {
  let mut s1 = String::new();
  io::stdin().read_line(&mut s1).expect("err");
  let mut c1: i64 = s1.trim().parse().expect("err");
  let mut s2 = String::new();
  io::stdin().read_line(&mut s2).expect("err");
  let mut c2: i64 = s2.trim().parse().expect("err");
  let mut arr: [i64; 10] = [0; 10];
  for i in 0..10 {
    let mut s3 = String::new();
    io::stdin().read_line(&mut s3).expect("err");
    arr[i] = s3.trim().parse().expect("err");
  }
  let mut b: bool = false;
  for i in 0..10 {
    if (c1 == arr[i]) {
      for j in i..10 {
        if (c2 == arr[j]) {
          arr[i] ^= arr[j];
          arr[j] ^= arr[i];
          arr[i] ^= arr[j];
          b = true;
          break;
        }
      }
    } else if (c2 == arr[i]) {
      for j in i..10 {
        if (c1 == arr[j]) {
          arr[i] ^= arr[j];
          arr[j] ^= arr[i];
          arr[i] ^= arr[j];
          b = true;
          break;
        }
      }
    }
    if (b) {
      break;
    }
  }
  println!("{:?}", arr);
}
