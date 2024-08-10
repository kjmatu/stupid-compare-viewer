// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::api::dialog::blocking::FileDialogBuilder;


#[tauri::command]
async fn open_image()  -> Result<String, String> {
  let dialog_result = FileDialogBuilder::new().pick_file();
  match dialog_result {
    Some(path) => {
      return Ok(path.display().to_string());
    },
    None => Ok("No file selected".to_string()),
  }
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![open_image])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
