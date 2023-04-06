use dotenvy::dotenv;
use std::process::Command;

pub fn main() {
  dotenv().ok();
  std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

  Command::new("sh")
    .arg("-C")
    .arg("build/build.sh")
    .spawn()
    .expect("build/build.sh failed to start");
}
