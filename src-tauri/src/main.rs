// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::api::dialog::blocking::FileDialogBuilder;
use serde::Serialize;
use std::path::PathBuf;
use std::fs::File;
use std::io::Read;
use base64::encode;


#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ImageData {
  file_path: PathBuf,
  base64_data: String,
  width: u32,
  height: u32,
}

#[tauri::command]
async fn open_image()  -> Result<ImageData, String> {
  let dialog_result = FileDialogBuilder::new().pick_file();
  match dialog_result {
    Some(path) => {
      let mut file = File::open(path.clone()).map_err(|e| e.to_string())?;
      let mut buffer = Vec::new();
      file.read_to_end(&mut buffer).map_err(|e| e.to_string())?;
      let image_data = ImageData {
        file_path: path,
        base64_data: encode(&buffer),
        width: 0,
        height: 0,
      };
      Ok(image_data)
    },
    None => Ok(ImageData{
      file_path: "".into(),
      base64_data: "".to_string(),
      width: 0,
      height: 0,
    }),
  }
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![open_image])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
