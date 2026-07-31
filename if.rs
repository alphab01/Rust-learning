use std::io;

fn main() {
  let mut s1 = String::new();
  io::stdin().read_line(&mut s1).expect("err");
  let mut c: i64 = s1.trim().parse().expect("err");
  let mut mi: i64 = 1000;
  let (mut c1, mut c2, mut c3) = (c/100, (c/10)%10, c%10);
  if (c1 != 0 && c2 != 0 && c3 != 0) {
    if (c1 * 100 + c2 * 10 + c3 < mi) {
      mi = c1 * 100 + c2 * 10 + c3;
    }
    if (c2 * 100 + c1 * 10 + c3 < mi) {
      mi = c2 * 100 + c1 * 10 + c3;
    }
    if (c3 * 100 + c2 * 10 + c1 < mi) {
      mi = c3 * 100 + c2 * 10 + c1;
    }
    if (c1 * 100 + c3 * 10 + c2 < mi) {
      mi = c1 * 100 + c3 * 10 + c2;
    }
    if (c3 * 100 + c1 * 10 + c2 < mi) {
      mi = c3 * 100 + c1 * 10 + c2;
    }
    if (c2 * 100 + c3 * 10 + c1 < mi) {
      mi = c2 * 100 + c3 * 10 + c1;
    }
  } else if (c1 == 0) {
    if (c2 != 0 && c3 != 0) {
      if (c2 * 100 + c3 < mi) {
        mi = c2 * 100 + c3;
      }
      if (c3 * 100 + c2 < mi) {
        mi = c3 * 100 + c2;
      }
    } else if (c2 == 0) {
      if (c3*100 < mi) {
        mi = c3 * 100;
      }
    } else {
      if (c2 * 100 < mi) {
        mi = c2 * 100;
      }
    }
  } else if (c2 == 0) {
    if (c1 != 0 && c3 != 0) {
      if (c1 * 100 + c3 < mi) {
        mi = c1 * 100 + c3;
      }
      if (c3 * 100 + c1 < mi) {
        mi = c3 * 100 + c1;
      }
    } else if (c1 == 0) {
      if (c3*100 < mi) {
        mi = c3 * 100;
      }
    } else {
      if (c1 * 100 < mi) {
        mi = c1 * 100;
      }
    }
  } else if (c3 == 0) {
    if (c2 != 0 && c1 != 0) {
      if (c2 * 100 + c1 < mi) {
        mi = c2 * 100 + c1;
      }
      if (c1 * 100 + c2 < mi) {
        mi = c1 * 100 + c2;
      }
    } else if (c2 == 0) {
      if (c1*100 < mi) {
        mi = c1 * 100;
      }
    } else {
      if (c2 * 100 < mi) {
        mi = c2 * 100;
      }
    }
  }
  println!("{mi}");
}
