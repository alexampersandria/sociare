use std::process::Command;

pub fn main() {
  Command::new("sh")
    .arg("-C")
    .arg("build/build.sh")
    .spawn()
    .expect("build/build.sh failed to start");
}
