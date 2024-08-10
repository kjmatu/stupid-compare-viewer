// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::api::dialog::blocking::FileDialogBuilder;
use serde::Serialize;
use std::path::PathBuf;
use std::fs::File;
use std::io::Read;
use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use ts_rs::TS;
use image::ImageReader;

#[derive(TS)]
#[ts(export)]
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct LoadedImageData {
  file_path: PathBuf,
  base64_data: String,
  width: u32,
  height: u32,
}

#[tauri::command]
async fn open_image()  -> Result<LoadedImageData, String> {
  let dialog_result = FileDialogBuilder::new().pick_file();
  match dialog_result {
    Some(path) => {
      let mut file = File::open(path.clone()).map_err(|e| e.to_string())?;
      let mut buffer = Vec::new();
      file.read_to_end(&mut buffer).map_err(|e| e.to_string())?;
      let img = ImageReader::open(&path).expect("Failed to open image").decode().expect("Failed to decode image");
      let image_data = LoadedImageData {
        file_path: path,
        base64_data: STANDARD.encode(&buffer),
        width: img.width(),
        height: img.height(),
      };
      Ok(image_data)
    },
    None => Ok(LoadedImageData{
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
