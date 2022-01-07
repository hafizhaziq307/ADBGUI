#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use std::process::Command;
use std::vec::Vec;
use tauri::command;

#[command]
fn get_packages() -> Vec<String> {
  let result = Command::new("sh")
    .arg("-c")
    .arg("adb shell pm list packages -s")
    .output()
    .expect("Failed to execute command");

  let str_packages: String = String::from_utf8(result.stdout).unwrap();

  let mut packages: Vec<String> = vec![];

  for package in str_packages.lines() {
    packages.push(package.to_string().replace("package:", ""));
  }
  packages.sort();

  return packages;
}

#[command]
fn delete_package(package: &str) -> () {
  Command::new("sh")
    .arg("-c")
    .arg("adb shell pm uninstall --user 0 ".to_owned() + package)
    .output()
    .expect("Failed to execute command");
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![get_packages, delete_package])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
