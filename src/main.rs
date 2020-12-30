use std::env;
use has_flag;

fn main() {
  // cargo run --package has_flag --bin has_flag
  const FLAG: &str = "run";

  println!("是否包含flag? {}", has_flag::has_flag(FLAG, env::args().collect()));
}
