#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod mathlib;
mod middleware;

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![middleware::test_operation,])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
