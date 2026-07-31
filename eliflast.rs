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
  if (c1 == 0) {
    println!("User (no access).");
  } else if (c1 == 1) {
    println!("User:\n    - execute only");
  } else if (c1 == 2) {
    println!("User:\n    - write only");
  } else if (c1 == 3) {
    println!("User:\n    - write\n    - execute");
  } else if (c1 == 4) {
    println!("User:\n    - read only");
  } else if (c1 == 5) {
    println!("User:\n    - read\n    - execute");
  } else if (c1 == 6) {
    println!("User:\n    - read\n    - write");
  } else if (c1 == 7) {
    println!("User (full access):\n    - read\n    - write\n    - execute");
  }
  if (c2 == 0) {
    println!("Group (no access).");
  } else if (c2 == 1) {
    println!("Group:\n    - execute only");
  } else if (c2 == 2) {
    println!("Group:\n    - write only");
  } else if (c2 == 3) {
    println!("Group:\n    - write\n    - execute");
  } else if (c2 == 4) {
    println!("Group:\n    - read only");
  } else if (c2 == 5) {
    println!("Group:\n    - read\n    - execute");
  } else if (c2 == 6) {
    println!("Group:\n    - read\n    - write");
  } else if (c2 == 7) {
    println!("Group (full access):\n    - read\n    - write\n    - execute");
  }
  if (c3 == 0) {
    println!("Other (no access).");
  } else if (c3 == 1) {
    println!("Other:\n    - execute only");
  } else if (c3 == 2) {
    println!("Other:\n    - write only");
  } else if (c3 == 3) {
    println!("Other:\n    - write\n    - execute");
  } else if (c3 == 4) {
    println!("Other:\n    - read only");
  } else if (c3 == 5) {
    println!("Other:\n    - read\n    - execute");
  } else if (c3 == 6) {
    println!("Other:\n    - read\n    - write");
  } else if (c3 == 7) {
    println!("Other (full access):\n    - read\n    - write\n    - execute");
  }
}
