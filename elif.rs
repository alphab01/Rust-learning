use std::io;

fn main() {
  let mut s1 = String::new();
  io::stdin().read_line(&mut s1).expect("err");
  let c1: i64 = s1.trim().parse().expect("err");
  let mut s2 = String::new();
  io::stdin().read_line(&mut s2).expect("err");
  let c2: i64 = s2.trim().parse().expect("err");
  let mut s3 = String::new();
  io::stdin().read_line(&mut s3).expect("err");
  let c3: i64 = s3.trim().parse().expect("err");
  if (17 < c1 && c1 < 40) {
    let mut a: bool = true;
    let mut b: bool = true;
    if (c2 < 90) {
      println!("Систолическое АД {0} ниже нормы на {1}", c2, 90 - c2);
      a = false;
    } else if (c2 > 139) {
      println!("Систолическое АД {0} выше нормы на {1}", c2, c2 - 139);
      a = false
    }
    if (c3 < 60) {
      if (a == true) {
        println!("Систолическое АД в норме");
        println!("Диастолическое АД {0} ниже нормы на {1}", c3, 60 - c3);
      } else {
        println!("Диастолическое АД {0} ниже нормы на {1}", c3, 60 - c3);
      }
      b = false;
    } else if (c3 > 89) {
      if (a == true) {
        println!("Систолическое АД в норме");
        println!("Диастолическое АД {0} выше нормы на {1}", c3, c3 - 89);
        b = false
      } else {
        println!("Диастолическое АД {0} выше нормы на {1}", c3, c3 - 89);
        b = false
      }
    }
    if (a == true && b == true) {
      println!("Систолическое и Диастолическое АД в норме");
    }
    else if (a == false && b == true) {
      println!("Диастолическое АД в норме");
    }
  } else if (39 < c1 && c1 < 60) {
    let mut a: bool = true;
    let mut b: bool = true;
    if (c2 < 91) {
      println!("Систолическое АД {0} ниже нормы на {1}", c2, 91 - c2);
      a = false;
    } else if (c2 > 149) {
      println!("Систолическое АД {0} выше нормы на {1}", c2, c2 - 149);
      a = false
    }
    if (c3 < 61) {
      if (a == true) {
        println!("Систолическое АД в норме");
        println!("Диастолическое АД {0} ниже нормы на {1}", c3, 61 - c3);
      } else {
        println!("Диастолическое АД {0} ниже нормы на {1}", c3, 61 - c3);
      }
      b = false;
    } else if (c3 > 91) {
      if (a == true) {
        println!("Систолическое АД в норме");
        println!("Диастолическое АД {0} выше нормы на {1}", c3, c3 - 91);
        b = false
      } else {
        println!("Диастолическое АД {0} выше нормы на {1}", c3, c3 - 91);
        b = false
      }
    }
    if (a == true && b == true) {
      println!("Систолическое и Диастолическое АД в норме");
    }
    else if (a == false && b == true) {
      println!("Диастолическое АД в норме");
    }
  } else {
    let mut a: bool = true;
    let mut b: bool = true;
    if (c2 < 91) {
      println!("Систолическое АД {0} ниже нормы на {1}", c2, 91 - c2);
      a = false;
    } else if (c2 > 159) {
      println!("Систолическое АД {0} выше нормы на {1}", c2, c2 - 159);
      a = false
    }
    if (c3 < 61) {
      if (a == true) {
        println!("Систолическое АД в норме");
        println!("Диастолическое АД {0} ниже нормы на {1}", c3, 61 - c3);
      } else {
        println!("Диастолическое АД {0} ниже нормы на {1}", c3, 61 - c3);
      }
      b = false;
    } else if (c3 > 91) {
      if (a == true) {
        println!("Систолическое АД в норме");
        println!("Диастолическое АД {0} выше нормы на {1}", c3, c3 - 91);
        b = false
      } else {
        println!("Диастолическое АД {0} выше нормы на {1}", c3, c3 - 91);
        b = false
      }
    }
    if (a == true && b == true) {
      println!("Систолическое и Диастолическое АД в норме");
    }
    else if (a == false && b == true) {
      println!("Диастолическое АД в норме");
    }
  }
}
