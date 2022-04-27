#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod frontend;
mod mathlib;
mod middleware;

/// Entry point
fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![middleware::math_operation])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
