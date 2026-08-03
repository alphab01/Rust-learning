use std::io;

fn main() {
  let mut arr: [i64; 10] = [0; 10];
  for i in 0..10 {
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("err");
    arr[i] = s.trim().parse().expect("err");
  }
  let mut i1: i64 = 0;
  let mut ma: i64 = 0;
  for i in 0..10 {
    let mut s: i64 = 1;
    let mut c = arr[i];
    for j in (i + 1)..10 {
      if (arr[j] <= c) {
        break;
      } else {
        s += 1;
        c = arr[j];
      }
    }
    if (s > ma) {
      ma = s;
      i1 = i as i64;
    }
  }
  for i in i1..(i1 + ma) {
    if (i != (i1 + ma - 1)) {
      print!("{} ", arr[i as usize]);
    } else {
      print!("{}\n", arr[i as usize]);
    }
  }
}
