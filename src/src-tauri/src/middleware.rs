use tauri::command;

#[command]
pub fn test_operation(event: String, payload: Option<String>) {
  println!("{} {:?}", event, payload);
}
