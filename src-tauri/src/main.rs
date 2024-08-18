// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::api::dialog::blocking::FileDialogBuilder;
use std::path::PathBuf;
use serde::Serialize;
use ts_rs::TS;

#[derive(TS)]
#[ts(export)]
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct LoadedImageData {
  file_path: PathBuf,
}

#[tauri::command]
async fn open_image()  -> Result<LoadedImageData, String> {
  let dialog_result = FileDialogBuilder::new().pick_file();
  match dialog_result {
    Some(path) => {
      let image_data = LoadedImageData {
        file_path: path,
      };
      Ok(image_data)
    },
    None => Ok(LoadedImageData{
      file_path: "".into(),
    }),
  }
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![open_image])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
