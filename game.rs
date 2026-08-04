use std::io;

fn main() {
  println!(" x  y  z");
  for x in -10..=10 {
    for y in -3..=3 {
      for z in 2..=6 {
        if (4 * x - 2 * y + 3 * z == 20 && x * y * z < 15 && x * x + y * y + z * z > 8) {
          if (x < 0 && y < 0 && z < 0) {
            println!("{x} {y} {z}");
          } else if (x >= 0 && y < 0 && z < 0) {
            println!(" {x} {y} {z}");
          } else if (x < 0 && y >= 0 && z < 0) {
            println!("{x}  {y} {z}");
          } else if (x < 0 && y < 0 && z >= 0) {
            println!("{x} {y}  {z}");
          } else if (x >= 0 && y >= 0 && z < 0) {
            println!(" {x}  {y} {z}");
          } else if (x >= 0 && y < 0 && z >= 0) {
            println!(" {x} {y}  {z}");
          } else if (x < 0 && y >= 0 && z >= 0) {
            println!("{x}  {y}  {z}");
          } else if (x >= 0 && y >= 0 && z >= 0) {
            println!(" {x}  {y}  {z}");
          }
        }
      }
    }
  }
}
