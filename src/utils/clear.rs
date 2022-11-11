use std::process::Command;

pub fn by_os() {
  let os = std::env::consts::OS;

  if os == "linux" || os == "macos" {
    Command::new("clear").status().unwrap();
  }
  if os == "windows" {
    Command::new("cls").status().unwrap();
  }
}