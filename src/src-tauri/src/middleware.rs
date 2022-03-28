use tauri::command;

#[command]
pub fn test_operation(event: String, payload: Option<String>) -> String {
  println!("{} {:?}", event, payload);
  "Pong!".into()
}
