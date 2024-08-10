// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::api::dialog::blocking::FileDialogBuilder;


#[tauri::command]
async fn greet()  -> Result<String, String> {
  let dialog_result = FileDialogBuilder::new().pick_file();
  println!("{:?}", dialog_result);
  Ok(dialog_result.unwrap().into_os_string().into_string().unwrap())
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![greet])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
